import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import '@/styles/globals.css'
import { cn } from '@/lib/utils'

export const fontSans = Inter({
  subsets: ['latin'],
  variable: "--font-sans",
})

export const metadata: Metadata = {
  title: 'Protin',
  description: 'Protin - Beefed up Text Storage Site',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className='dark'>
      <body className={cn(
        "min-h-screen bg-background font-sans antialiased",
        fontSans.variable
      )}>{children}</body>
    </html>
  )
}
