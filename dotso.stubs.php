<?php

/**
 * Stubs for documentation.
 */

namespace Dotso;

class Response
{
    function status()
    {
    }

    function headers()
    {
    }

    function body()
    {
    }
}


class HttpClientBuilder
{
    public function __construct()
    {
    }

    public function timeout(int $timeout): object
    {
        return $this;
    }

    public function cookie_store(bool $store)
    {
    }

    public function build()
    {
    }
}

class CouchDB
{
    public function __construct(string $url, $client)
    {
    }

    public function get(string $database, string $id)
    {
    }
}
