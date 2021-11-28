type DebugFn = (...args: any[]) => void;

export const scopedDebug = (scope: string): DebugFn => {
  return (...args: any[]) => {
    import.meta.env.DEV && console.log(`[${scope}]`, ...args);
  };
};
