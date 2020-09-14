FORMAT: 1A
HOST: https://api.airdrome.io/

# Airdrome
Airdrome is a repository for objects used by the Parallax family of
microcontrollers.

## Group Objects

Resources related to OBEX objects.

## Objects Collection [/objects]

### List All Objects [GET /objects{?limit}{&created,updated,name,targets,languages,keywords,categories}]

Returns a list of objects. Allows for pagination and searching using the
optional parameters.

+ Parameters

  + limit: `` (integer, optional) - maximum number of objects to return

    Maximum number of objects to return.

    + Default: `50`

  + created:[lt,lte,gt,gte]: `2020-08-30` (string, optional) - date object was created

    Date needs to be ISO 8601 formatted. The `lt` (less than), `lte`
    (less than or equal to), `gt` (greater than), or `gte`
    (greater than or equal to) set how the date is used in searching for
    objects.

  + updated:[lt,lte,gt,gte]: `2020-08-30` (string, optional) - date object was last updated

    Date needs to be ISO 8601 formatted. The `lt` (less than), `lte`
    (less than or equal to), `gt` (greater than), or `gte`
    (greater than or equal to) set how the date is used in searching for
    objects.

  + name `corrosion` (string, optional) - name of the object

  + targets: `p1,p2` (enum[string], optional) - list of compatiable microcontrollers

    Comma deliminated list of microcontrollers that the object is designed for.
    One or more targets can be provided.

    + Members
      + `bs1`
      + `bs2`
      + `bs2e`
      + `bs2sx`
      + `bs2p24`
      + `bs2p40`
      + `bs2pe`
      + `bs2px`
      + `sx`
      + `p1`
      + `p2`

  + languages: `pasm,pasm2` (enum[string], optional) - list of languages used in object

    Comma deliminated list of languages used in the object.
    One or more languages can be provided.

    + Members
      + `spin`
      + `spin2`
      + `pasm`
      + `pasm2`
      + `c`
      + `basic`
      + `forth`
      + `python`

  + keywords: `` (string, optional) - list of keywords associated with the object

    Comma deliminated list of keywords to use when searching for objects.

  + categories: `` (enum[string], optional) - comma deliminated list of categories

    Comma deliminated of object categories. The categories are the same ones
    used in the official Parallax OBEX repository.

    + Members
      + ``

+ Request

+ Response 200 (applicatioin/json)
  [{
    guid: "357b0c1c-81e3-47fd-9220-0c21cbf76396",
    name: "corrosion",
    license: "MIT",
    readme: "base64 encoded contents of README.md",
    website: "https://github.com/emiljt/corrosion",
    documentation: "https://github.com/emiljt/corrosion/blob/master/documentation.md",
    authors: [{
      name: "Joshua Terrasas",
      email: "jcterrasas@gmail.com",
      website: "https://github.com/emiljt"
    }],
    versions: [{
      number: "v0.1.0",
      created: "2019-09-15",
      zip: {
        size: 1234,
        link: "https://airdrome.io/downloads/9b75b8ef-f58c-4cc0-8642-ab8405066eab"
      }
    }],
    targets: ["p2"],
    stats: [{
      name: "Total Downloads",
      value: "150",
      updated: "2020-08-30"
    }],
    categories: ["database"]
  }]

## Object [/objects/{object_guid}]

### Get a Specific Object [GET]

Returns a specific object.

+ Parameters
  + object_guid `357b0c1c-81e3-47fd-9220-0c21cbf76396` (string, required) - Guid of the object

+ Request

+ Response 200 (applicatioin/json)
  [{
    guid: "357b0c1c-81e3-47fd-9220-0c21cbf76396",
    name: "corrosion",
    license: "MIT",
    readme: "base64 encoded contents of README.md",
    website: "https://github.com/emiljt/corrosion",
    documentation: "https://github.com/emiljt/corrosion/blob/master/documentation.md",
    authors: [{
      name: "Joshua Terrasas",
      email: "jcterrasas@gmail.com",
      website: "https://github.com/emiljt"
    }],
    versions: [{
      number: "v0.1.0",
      created: "2019-09-15",
      zip: {
        size: 1234,
        link: "https://airdrome.io/downloads/9b75b8ef-f58c-4cc0-8642-ab8405066eab"
      }
    }],
    microcontroller: "p2",
    stats: [{
      name: "Total Downloads",
      value: "150",
      updated: "2020-08-30"
    }],
    categories: ["database"]
  }]

## Group Stats

Resources related to Airdrome statistics.

## Stats Collection [/stats]

### List All Stats [GET]

Returns all of the latest stats for Airdrome.

+ Request

+ Response 200 (application/json)
