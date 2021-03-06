// Magical Bitcoin Library
// Written in 2020 by
//     Alekos Filini <alekos.filini@gmail.com>
//
// Copyright (c) 2020 Magical Bitcoin
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Descriptor errors

/// Errors related to the parsing and usage of descriptors
#[derive(Debug)]
pub enum Error {
    //InternalError,
    //InvalidPrefix(Vec<u8>),
    //HardenedDerivationOnXpub,
    //MalformedInput,
    /// Invalid HD Key path, such as having a wildcard but a length != 1
    InvalidHDKeyPath,

    //KeyParsingError(String),
    #[allow(missing_docs)]
    Key(crate::keys::KeyError),
    #[allow(missing_docs)]
    Policy(crate::descriptor::policy::PolicyError),

    //InputIndexDoesntExist,
    //MissingPublicKey,
    //MissingDetails,
    /// Invalid character found in the descriptor checksum
    InvalidDescriptorCharacter(char),

    //CantDeriveWithMiniscript,
    #[allow(missing_docs)]
    BIP32(bitcoin::util::bip32::Error),
    #[allow(missing_docs)]
    Base58(bitcoin::util::base58::Error),
    #[allow(missing_docs)]
    PK(bitcoin::util::key::Error),
    #[allow(missing_docs)]
    Miniscript(miniscript::Error),
    #[allow(missing_docs)]
    Hex(bitcoin::hashes::hex::Error),
}

impl From<crate::keys::KeyError> for Error {
    fn from(key_error: crate::keys::KeyError) -> Error {
        match key_error {
            crate::keys::KeyError::Miniscript(inner) => Error::Miniscript(inner),
            crate::keys::KeyError::BIP32(inner) => Error::BIP32(inner),
            e => Error::Key(e),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl_error!(bitcoin::util::bip32::Error, BIP32);
impl_error!(bitcoin::util::base58::Error, Base58);
impl_error!(bitcoin::util::key::Error, PK);
impl_error!(miniscript::Error, Miniscript);
impl_error!(bitcoin::hashes::hex::Error, Hex);
impl_error!(crate::descriptor::policy::PolicyError, Policy);
