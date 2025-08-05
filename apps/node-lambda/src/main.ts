import { DynamoDB } from 'aws-sdk';
import { APIGatewayProxyHandler } from 'aws-lambda';

const dynamo = new DynamoDB.DocumentClient();
const tableName = process.env.DYNAMO_TABLE!;

export const handler: APIGatewayProxyHandler = async (event) => {
  try {
    const name = event.name;

    if (!name) {
      return {
        statusCode: 400,
        body: JSON.stringify({ error: 'Missing "name" in request body' }),
      };
    }

    const timestamp = new Date().toISOString();

    await dynamo
      .put({
        TableName: tableName,
        Item: {
          name,
          timestamp,
        },
      })
      .promise();

    console.info(`Successfully wrote to DynamoDB: ${name}`);

    return {
      statusCode: 200,
      body: JSON.stringify({ message: `Hello, ${name}!` }),
    };
  } catch (err) {
    console.error('Error processing request:', err);
    return {
      statusCode: 500,
      body: JSON.stringify({ error: 'Internal Server Error' }),
    };
  }
};
