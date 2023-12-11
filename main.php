<?php

use Dotso\CouchDB;

$couchDB = new CouchDB(databaseHost: '127.0.0.1:5984');

var_dump($couchDB);
