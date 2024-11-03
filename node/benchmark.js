// benchmark.js
const { Client } = require('pg');

const client = new Client({
  user: 'user',
  host: 'localhost',
  database: 'benchmark_db',
  password: 'password',
  port: 5432,
});

client.query = (originalQuery => {
  return (...args) => {
    const queryText = args[0];
    const values = args[1];

    console.log('Executing query:', queryText);
    if (values) {
      console.log('With values:', values);
    }

    // Call the original query method
    return originalQuery.apply(client, args);
  };
})(client.query);



const addRecord = async (client, value) => {
  const query = `INSERT INTO benchmark (value) VALUES ($1)`;
  const params = [value];
  return client.query(query, params);
}

const batchInsertSingleStatementRecord = async (batchSize, totalCount) => {
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

async function batchInsertMultipleStatementsMultipleRecord(batchSize, totalCount) {

  await client.connect();

  for (let i = 0; i < totalCount; i += batchSize) {
      const params = Array.from({ length: Math.min(batchSize, totalCount - i) }, (_, j) => `Row ${i + j + 1}`);
      const values = params.map((_, idx) => `($${idx + 1})`).join(',');
      const query = `INSERT INTO benchmark (value) VALUES ${values}`;
      
      console.time(`Insert Batch ${i / batchSize + 1}`);
      await Promise.allSettled(queries);
      console.timeEnd(`Insert Batch ${i / batchSize + 1}`);
  }

  await client.end();
}

async function batchInsertSingleStatementMultipleRecord(batchSize, totalCount) {
  await client.connect();

  for (let i = 0; i < totalCount; i += batchSize) {
      const params = [];
      const values = [];

      for (let j = 0; j < Math.min(batchSize, totalCount - i); j++) {
          params.push(`Row ${i + j + 1}`);
          values.push(`($${j + 1})`);
      }

      const query = `INSERT INTO benchmark (value) VALUES ${values.join(',')}`;
      const startTime = Date.now();

      await client.query(query, params);
      console.log(`Insert Batch ${Math.floor(i / batchSize) + 1}: ${Date.now() - startTime}ms`);
  }

  await client.end();
}

const batchInsertMultipleStatements = async (batchSize, totalCount) => {
  await client.connect();

  for (let i = 0; i < totalCount; i += batchSize) {
    const queries = Array.from({ length: Math.min(batchSize, totalCount - i) }, (_, j) => addRecord(client, `Row ${i + j + 1}`));
    
    console.time(`Insert Batch ${i / batchSize + 1}`);
    await Promise.allSettled(queries);
    console.timeEnd(`Insert Batch ${i / batchSize + 1}`);
  }

  await client.end();
}

const clearRecords = async () => {
  await client.connect();
  await client.query('DELETE FROM benchmark');
  await client.end();
};

module.exports = { batchInsertMultipleStatements, batchInsertMultipleStatements, clearRecords };

// Usage: node benchmark.js
//batchInsertSingleStatementMultipleRecord(10, 200000).then(() => console.log("Batch insert complete"));
batchInsertMultipleStatements(10, 200000).then(() => console.log("Batch insert complete"));
//batchInsertMultipleStatementsMultipleRecord(10, 200000).then(() => console.log("Batch insert complete"));