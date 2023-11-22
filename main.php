<?php

use HttpClient\HttpClientBuilder;
use HttpClient\HttpClientException;

$client = (new HttpClientBuilder())->timeout(15000)->cookie_store(true)->build();

$response = $client->get('https://google.com/')->send();
var_dump([
    'status' => $response->status(),
    'headers' => $response->headers(),
]);

// Prints: builder error for url (file:///): URL scheme is not allowed
try {
    $client->get('file:///')->send();
} catch (HttpClientException $e) {
    var_dump($e->getMessage());
}
