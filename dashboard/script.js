const resultsPath = 'data/results.json';
const fallbackPath = 'data/sample_results.json';

async function loadResults() {
  const response = await fetchWithFallback(resultsPath, fallbackPath);
  const data = await response.json();
  return Array.isArray(data) ? data : [data];
}

async function fetchWithFallback(primary, fallback) {
  try {
    const res = await fetch(primary, { cache: 'no-store' });
    if (!res.ok) throw new Error('Primary fetch failed');
    return res;
  } catch (err) {
    return fetch(fallback, { cache: 'no-store' });
  }
}

function formatNumber(value) {
  return value.toLocaleString(undefined, { maximumFractionDigits: 2 });
}

function formatWork(result) {
  return `${result.work_label}: ${formatNumber(result.work_units)}`;
}

function renderTable(results) {
  const tbody = document.getElementById('runs-table');
  tbody.innerHTML = '';
  results
    .slice()
    .sort((a, b) => new Date(b.timestamp_utc) - new Date(a.timestamp_utc))
    .forEach((result) => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td>${new Date(result.timestamp_utc).toISOString()}</td>
        <td>${result.mode}</td>
        <td>${formatWork(result)}</td>
        <td>${result.pi_estimate.toFixed(6)}</td>
        <td>${result.elapsed_seconds.toFixed(3)} s</td>
        <td>${formatNumber(result.throughput_per_second)} units/s</td>
        <td>${result.absolute_error.toExponential(2)}</td>
        <td>${result.notes ? result.notes : ''}</td>
      `;
      tbody.appendChild(tr);
    });
}

function renderSummary(results) {
  const totalRuns = results.length;
  const avgThroughput =
    results.reduce((acc, r) => acc + (r.throughput_per_second || 0), 0) / totalRuns;
  const avgError = results.reduce((acc, r) => acc + (r.absolute_error || 0), 0) / totalRuns;

  document.getElementById('total-runs').textContent = totalRuns;
  document.getElementById('avg-throughput').textContent =
    totalRuns > 0 ? `${formatNumber(avgThroughput)} units/s` : '-';
  document.getElementById('avg-error').textContent =
    totalRuns > 0 ? avgError.toExponential(2) : '-';
}

function renderLatest(results) {
  if (!results.length) return;
  const latest = results.slice().sort((a, b) => new Date(b.timestamp_utc) - new Date(a.timestamp_utc))[0];
  document.getElementById('latest-run').textContent = `${latest.mode} — ${formatWork(latest)} (${latest.elapsed_seconds.toFixed(3)} s)`;
  document.getElementById('latest-notes').textContent = latest.notes || '';

  const profile = latest.system || {};
  const items = [
    ['OS', profile.os_name],
    ['Kernel', profile.kernel_version],
    ['Architecture', profile.cpu_architecture],
    ['CPU', profile.cpu_model],
    ['CPU freq', profile.cpu_frequency_mhz ? `${profile.cpu_frequency_mhz} MHz` : null],
    ['Cores (logical)', profile.logical_cores],
    ['Cores (physical)', profile.physical_cores],
    ['RAM total', profile.total_memory_bytes ? formatBytes(profile.total_memory_bytes) : null],
    ['RAM available', profile.available_memory_bytes ? formatBytes(profile.available_memory_bytes) : null],
    ['Hardware guess', profile.hardware_type_guess],
  ]
    .filter(([, value]) => value !== null && value !== undefined)
    .map(([label, value]) => `<li><span>${label}</span><strong>${value}</strong></li>`) // minimal HTML injection risk, data is trusted from repo
    .join('');

  document.getElementById('system-profile').innerHTML = items || '<li class="muted">No system details</li>';
}

function formatBytes(bytes) {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let value = bytes;
  let unitIndex = 0;
  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }
  return `${value.toFixed(1)} ${units[unitIndex]}`;
}

function renderChart(results) {
  const ctx = document.getElementById('throughputChart').getContext('2d');
  const byMode = results.reduce((acc, r) => {
    acc[r.mode] = acc[r.mode] || [];
    acc[r.mode].push(r);
    return acc;
  }, {});

  const colors = ['#6ea7ff', '#9c66ff', '#4ad991', '#f5a524'];
  const datasets = Object.entries(byMode).map(([mode, runs], index) => {
    const sorted = runs.slice().sort((a, b) => new Date(a.timestamp_utc) - new Date(b.timestamp_utc));
    return {
      label: mode,
      data: sorted.map((r) => ({ x: new Date(r.timestamp_utc), y: r.throughput_per_second })),
      tension: 0.25,
      borderColor: colors[index % colors.length],
      backgroundColor: colors[index % colors.length],
      pointRadius: 3,
    };
  });

  new Chart(ctx, {
    type: 'line',
    data: { datasets },
    options: {
      responsive: true,
      plugins: {
        legend: { display: false },
        tooltip: {
          callbacks: {
            label: (context) => `${context.dataset.label}: ${formatNumber(context.parsed.y)} units/s`,
          },
        },
      },
      scales: {
        x: {
          type: 'time',
          time: { unit: 'day' },
          ticks: { color: '#9aa6b2' },
        },
        y: {
          ticks: {
            color: '#9aa6b2',
            callback: (value) => formatNumber(value),
          },
          grid: { color: '#1f2736' },
        },
      },
    },
  });

  const legend = document.getElementById('legend');
  legend.innerHTML = datasets
    .map((dataset) => `
      <span class="legend-item">
        <span class="swatch" style="background:${dataset.borderColor}"></span>
        ${dataset.label}
      </span>
    `)
    .join('');
}

(async function init() {
  const results = await loadResults();
  renderTable(results);
  renderSummary(results);
  renderLatest(results);
  renderChart(results);
})();
