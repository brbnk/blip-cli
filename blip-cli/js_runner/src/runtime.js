const { core } = Deno

globalThis.context = {
    getVariableAsync: async function(key) {
        return await core.ops.op_get_var(key);
    },
    setVariableAsync: async function(key, value) {
        try {
            await core.ops.op_set_var(key, JSON.stringify(value))
        } catch(e) {
            await core.ops.op_set_var(key, value)
        }
        
    },
    deleteVariableAsync: async function(key) {
        await core.ops.op_delete_var(key)
    }
}