/*
 * Copyright 2024 5ohue
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::ops::{Deref, DerefMut};
use crate::bindings;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GeometryInfo(bindings::GeometryInfo);
impl GeometryInfo {
    pub fn new() -> GeometryInfo {
        let inner_geometry_info =  bindings::GeometryInfo {
            rho: 0.0,
            sigma: 0.0,
            xi: 0.0,
            psi: 0.0,
            chi: 0.0,
        };

        Self(inner_geometry_info)
    }

    pub fn set_rho(&mut self, rho: f64) {
        self.rho = rho;
    }
    pub fn set_sigma(&mut self, sigma: f64) {
        self.sigma = sigma;
    }
    pub fn set_xi(&mut self, xi: f64) {
        self.xi = xi;
    }
    pub fn set_psi(&mut self, psi: f64) {
        self.psi = psi;
    }
    pub fn set_chi(&mut self, chi: f64) {
        self.chi = chi;
    }
}

impl DerefMut for GeometryInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for GeometryInfo {
    type Target = bindings::GeometryInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for GeometryInfo {
    fn default() -> Self {
        Self::new()
    }
}
