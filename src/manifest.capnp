@0x847d50a412ae7c63;

using Package = import "package.capnp";

const manifest :Package.Manifest = (
  appVersion = 0,

  actions = [(

    input = (none = void),
    title = (defaultText = "New Acronymy Instance"),

    command = (
      executablePath = "/acronymy"
    )
  )],

  continueCommand = (
    executablePath = "/acronymy"
  )
);