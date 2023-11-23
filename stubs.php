<?php

/**
 * Stubs for documentation.
 */

namespace HttpClient;

class HttpClientBuilder
{
    function timeout(int $ms): HttpClientBuilder
    {
        return $this;
    }

    function cookie_store(bool $enable): HttpClientBuilder
    {
        return $this;
    }

    function build(): Client
    {
        return new Client;
    }
}
class HttpClientException
{
    function timeout()
    {
    }

    function getMessage(): string
    {
        return '';
    }
}
class Client
{
    function get(string $url)
    {
        return new RequestBuilder;
    }

    function post(string $url)
    {
        return new RequestBuilder;
    }
}

class RequestBuilder
{
    function send()
    {
        return new Response;
    }
}

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
