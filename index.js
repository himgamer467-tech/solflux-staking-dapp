import Link from "next/link";

export default function Home() {
  return (
    <div className="container">
      <h1>SolFlux NFT Yield Platform</h1>
      <p>Stake your NFTs and earn yield on Solana.</p>

      <Link href="/stake">
        <button>Start Staking</button>
      </Link>
    </div>
  );
}
