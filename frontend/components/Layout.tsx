import { EventHandler } from "./EventHandler"

export interface LayoutProps {
  children: React.ReactNode
}

export const Layout = ({ children }: LayoutProps) => {
  return (
    <>
      <EventHandler />
      {children}
    </>
  )
}
