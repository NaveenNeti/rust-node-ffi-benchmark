// benchmark.js
const { Client } = require('pg');

const client = new Client({
  user: 'user',
  host: 'localhost',
  database: 'benchmark_db',
  password: 'password',
  port: 5432,
});

const batchInsert = async (batchSize, totalCount) => {
  await client.connect();

  for (let i = 0; i < totalCount; i += batchSize) {
    const params = Array.from({ length: Math.min(batchSize, totalCount - i) }, (_, j) => `Row ${i + j + 1}`);
    const values = params.map((_, idx) => `($${idx + 1})`).join(',');
    const query = `INSERT INTO benchmark (value) VALUES ${values}`;
    
    console.time(`Insert Batch ${i / batchSize + 1}`);
    await client.query(query, params);
    console.timeEnd(`Insert Batch ${i / batchSize + 1}`);
  }

  await client.end();
};

const clearRecords = async () => {
  await client.connect();
  await client.query('DELETE FROM benchmark');
  await client.end();
};

module.exports = { batchInsert, clearRecords };

// Usage: node benchmark.js
batchInsert(100, 6000000).then(() => console.log("Batch insert complete"));