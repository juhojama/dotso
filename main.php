<?php

use Dotso\CouchDB;
use Dotso\HttpClientBuilder;

$client = (new HttpClientBuilder())
    ->timeout(15000)
    ->cookie_store(true)
    ->build();

$couchDB = new CouchDB(url: 'http://127.0.0.1:5984', client: $client);
$result = $couchDB->get(database: 'users', id: '123');
var_dump($result);
