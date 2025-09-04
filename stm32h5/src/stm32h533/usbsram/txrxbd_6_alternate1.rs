///Register `TXRXBD_6_ALTERNATE1` reader
pub type R = crate::R<TXRXBD_6_ALTERNATE1rs>;
///Register `TXRXBD_6_ALTERNATE1` writer
pub type W = crate::W<TXRXBD_6_ALTERNATE1rs>;
///Field `ADDR_RX` reader - Reception buffer address
pub type ADDR_RX_R = crate::FieldReader<u16>;
///Field `ADDR_RX` writer - Reception buffer address
pub type ADDR_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `COUNT_RX` reader - Reception byte count
pub type COUNT_RX_R = crate::FieldReader<u16>;
///Field `NUM_BLOCK` reader - Number of blocks
pub type NUM_BLOCK_R = crate::FieldReader;
///Field `NUM_BLOCK` writer - Number of blocks
pub type NUM_BLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BLSIZE` reader - Block size
pub type BLSIZE_R = crate::BitReader;
///Field `BLSIZE` writer - Block size
pub type BLSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Reception buffer address
    #[inline(always)]
    pub fn addr_rx(&self) -> ADDR_RX_R {
        ADDR_RX_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:25 - Reception byte count
    #[inline(always)]
    pub fn count_rx(&self) -> COUNT_RX_R {
        COUNT_RX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 26:30 - Number of blocks
    #[inline(always)]
    pub fn num_block(&self) -> NUM_BLOCK_R {
        NUM_BLOCK_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Block size
    #[inline(always)]
    pub fn blsize(&self) -> BLSIZE_R {
        BLSIZE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXRXBD_6_ALTERNATE1")
            .field("addr_rx", &self.addr_rx())
            .field("count_rx", &self.count_rx())
            .field("num_block", &self.num_block())
            .field("blsize", &self.blsize())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Reception buffer address
    #[inline(always)]
    pub fn addr_rx(&mut self) -> ADDR_RX_W<TXRXBD_6_ALTERNATE1rs> {
        ADDR_RX_W::new(self, 0)
    }
    ///Bits 26:30 - Number of blocks
    #[inline(always)]
    pub fn num_block(&mut self) -> NUM_BLOCK_W<TXRXBD_6_ALTERNATE1rs> {
        NUM_BLOCK_W::new(self, 26)
    }
    ///Bit 31 - Block size
    #[inline(always)]
    pub fn blsize(&mut self) -> BLSIZE_W<TXRXBD_6_ALTERNATE1rs> {
        BLSIZE_W::new(self, 31)
    }
}
/**Channel/endpoint receive buffer descriptor 6

You can [`read`](crate::Reg::read) this register and get [`txrxbd_6_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_6_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USBSRAM:TXRXBD_6_ALTERNATE1)*/
pub struct TXRXBD_6_ALTERNATE1rs;
impl crate::RegisterSpec for TXRXBD_6_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`txrxbd_6_alternate1::R`](R) reader structure
impl crate::Readable for TXRXBD_6_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`txrxbd_6_alternate1::W`](W) writer structure
impl crate::Writable for TXRXBD_6_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXRXBD_6_ALTERNATE1 to value 0
impl crate::Resettable for TXRXBD_6_ALTERNATE1rs {}
