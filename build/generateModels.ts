import {knex} from 'knex';
import {updateTypes} from 'knex-types';

const db = knex({
  client: 'pg',
  connection: process.env.DISCPROV_DB_CONNECTION_STRING,
});

updateTypes(db, {output: './src/models.ts'}).catch(err => {
  console.error(err);
  process.exit(1);
});
