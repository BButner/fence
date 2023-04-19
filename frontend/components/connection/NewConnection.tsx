export const NewConnection: React.FC = () => {
  return (
    <div className="relative flex h-full w-full items-center justify-center">
      <div className="absolute left-0 top-0 z-0 h-96 w-96 -translate-x-1/2 rotate-45 border-4 border-gray-400 bg-gray-300/50 transition-all hover:border-red-400 hover:bg-red-300/50"></div>
      <div className="absolute bottom-0 right-0 z-0 h-96 w-96 translate-x-1/2 rotate-45 border-4 border-gray-400 bg-gray-300/50 transition-all hover:border-red-400 hover:bg-red-300/50"></div>
      <div className="z-10 space-y-12">
        <h1 className="bg-gradient-to-br from-violet-500 to-indigo-400 bg-clip-text fill-transparent text-center text-8xl font-semibold tracking-wide text-transparent">
          Fence
        </h1>
        <div className="flex items-center space-x-4">
          <input
            type="text"
            className="h-14 w-96 rounded-lg border-4 border-indigo-500 px-2 text-2xl shadow-xl outline-none transition-all focus:ring-8 focus:ring-indigo-500/50 dark:bg-gray-950 dark:text-white"
          />
          <button className="h-14	rounded-lg bg-indigo-500 px-8 text-xl text-white shadow-xl outline-none transition-all hover:bg-indigo-600 focus:ring-8 focus:ring-indigo-500/50">
            Connect
          </button>
        </div>
      </div>
    </div>
  )
}
