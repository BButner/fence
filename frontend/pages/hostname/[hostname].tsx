import { useRouter } from "next/router"

export default function Hostname() {
  const router = useRouter()
  const { hostname } = router.query

  return <div className="">{hostname}</div>
}
