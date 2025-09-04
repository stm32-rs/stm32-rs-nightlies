///Register `COUNT0_RX` reader
pub type R = crate::R<COUNT0_RXrs>;
///Register `COUNT0_RX` writer
pub type W = crate::W<COUNT0_RXrs>;
///Field `COUNT0_RX` reader - Reception byte count
pub type COUNT0_RX_R = crate::FieldReader<u16>;
///Field `NUM_BLOCK` reader - Number of blocks
pub type NUM_BLOCK_R = crate::FieldReader;
///Field `NUM_BLOCK` writer - Number of blocks
pub type NUM_BLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BL_SIZE` reader - Block size
pub type BL_SIZE_R = crate::BitReader;
///Field `BL_SIZE` writer - Block size
pub type BL_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - Reception byte count
    #[inline(always)]
    pub fn count0_rx(&self) -> COUNT0_RX_R {
        COUNT0_RX_R::new(self.bits & 0x03ff)
    }
    ///Bits 10:14 - Number of blocks
    #[inline(always)]
    pub fn num_block(&self) -> NUM_BLOCK_R {
        NUM_BLOCK_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bit 15 - Block size
    #[inline(always)]
    pub fn bl_size(&self) -> BL_SIZE_R {
        BL_SIZE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT0_RX")
            .field("count0_rx", &self.count0_rx())
            .field("num_block", &self.num_block())
            .field("bl_size", &self.bl_size())
            .finish()
    }
}
impl W {
    ///Bits 10:14 - Number of blocks
    #[inline(always)]
    pub fn num_block(&mut self) -> NUM_BLOCK_W<COUNT0_RXrs> {
        NUM_BLOCK_W::new(self, 10)
    }
    ///Bit 15 - Block size
    #[inline(always)]
    pub fn bl_size(&mut self) -> BL_SIZE_W<COUNT0_RXrs> {
        BL_SIZE_W::new(self, 15)
    }
}
/**Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count0_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count0_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:COUNT0_RX)*/
pub struct COUNT0_RXrs;
impl crate::RegisterSpec for COUNT0_RXrs {
    type Ux = u16;
}
///`read()` method returns [`count0_rx::R`](R) reader structure
impl crate::Readable for COUNT0_RXrs {}
///`write(|w| ..)` method takes [`count0_rx::W`](W) writer structure
impl crate::Writable for COUNT0_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COUNT0_RX to value 0
impl crate::Resettable for COUNT0_RXrs {}
