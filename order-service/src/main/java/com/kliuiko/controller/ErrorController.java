package com.kliuiko.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class ErrorController {

    @GetMapping("/error")
    public String getErrorResponse() {
        throw new Error("Unexpected problem occurred, hope app dynamics will help!");
    }
}
