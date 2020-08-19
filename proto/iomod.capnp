@0x8a75e17e00235777;

interface Agent {}

interface Iomod {
    getDeclaration @0 () -> (decl: Declaration);

    struct Declaration {
        organization @0 :Text;
        namespace    @1 :Text;
        name         @2 :Text;
    }
}

interface Registry {
    register @0 (iomod: Iomod) -> (agent: Agent);
}
