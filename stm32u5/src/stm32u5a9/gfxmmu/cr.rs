///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `B0OIE` reader - Buffer 0 overflow interrupt enable This bit enables the buffer 0 overflow interrupt.
pub type B0OIE_R = crate::BitReader;
///Field `B0OIE` writer - Buffer 0 overflow interrupt enable This bit enables the buffer 0 overflow interrupt.
pub type B0OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B1OIE` reader - Buffer 1 overflow interrupt enable This bit enables the buffer 1 overflow interrupt.
pub type B1OIE_R = crate::BitReader;
///Field `B1OIE` writer - Buffer 1 overflow interrupt enable This bit enables the buffer 1 overflow interrupt.
pub type B1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2OIE` reader - Buffer 2 overflow interrupt enable This bit enables the buffer 2 overflow interrupt.
pub type B2OIE_R = crate::BitReader;
///Field `B2OIE` writer - Buffer 2 overflow interrupt enable This bit enables the buffer 2 overflow interrupt.
pub type B2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B3OIE` reader - Buffer 3 overflow interrupt enable This bit enables the buffer 3 overflow interrupt.
pub type B3OIE_R = crate::BitReader;
///Field `B3OIE` writer - Buffer 3 overflow interrupt enable This bit enables the buffer 3 overflow interrupt.
pub type B3OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AMEIE` reader - AHB master error interrupt enable This bit enables the AHB master error interrupt.
pub type AMEIE_R = crate::BitReader;
///Field `AMEIE` writer - AHB master error interrupt enable This bit enables the AHB master error interrupt.
pub type AMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BM192` reader - 192 Block mode This bit defines the number of blocks per line
pub type BM192_R = crate::BitReader;
///Field `BM192` writer - 192 Block mode This bit defines the number of blocks per line
pub type BM192_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CE` reader - Cache enable This bit enables the cache unit.
pub type CE_R = crate::BitReader;
///Field `CE` writer - Cache enable This bit enables the cache unit.
pub type CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CL` reader - Cache lock This bit lock the cache onto the buffer defined in the CLB field.
pub type CL_R = crate::BitReader;
///Field `CL` writer - Cache lock This bit lock the cache onto the buffer defined in the CLB field.
pub type CL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLB` reader - Cache lock buffer This field select the buffer on which the cache is locked.
pub type CLB_R = crate::FieldReader;
///Field `CLB` writer - Cache lock buffer This field select the buffer on which the cache is locked.
pub type CLB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FC` reader - Force caching This bit force the caching into the cache regardless of the MPU attributes. The cache must be enable (CE bit set).
pub type FC_R = crate::BitReader;
///Field `FC` writer - Force caching This bit force the caching into the cache regardless of the MPU attributes. The cache must be enable (CE bit set).
pub type FC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD` reader - Prefetch disable This bit disables the prefetch of the cache.
pub type PD_R = crate::BitReader;
///Field `PD` writer - Prefetch disable This bit disables the prefetch of the cache.
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC` reader - Outter cachability This bit configure the cachability of an access generated by the GFXMMU cache.
pub type OC_R = crate::BitReader;
///Field `OC` writer - Outter cachability This bit configure the cachability of an access generated by the GFXMMU cache.
pub type OC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OB` reader - Outter bufferability This bit configure the bufferability of an access generated by the GFXMMU cache.
pub type OB_R = crate::BitReader;
///Field `OB` writer - Outter bufferability This bit configure the bufferability of an access generated by the GFXMMU cache.
pub type OB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Buffer 0 overflow interrupt enable This bit enables the buffer 0 overflow interrupt.
    #[inline(always)]
    pub fn b0oie(&self) -> B0OIE_R {
        B0OIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable This bit enables the buffer 1 overflow interrupt.
    #[inline(always)]
    pub fn b1oie(&self) -> B1OIE_R {
        B1OIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable This bit enables the buffer 2 overflow interrupt.
    #[inline(always)]
    pub fn b2oie(&self) -> B2OIE_R {
        B2OIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable This bit enables the buffer 3 overflow interrupt.
    #[inline(always)]
    pub fn b3oie(&self) -> B3OIE_R {
        B3OIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHB master error interrupt enable This bit enables the AHB master error interrupt.
    #[inline(always)]
    pub fn ameie(&self) -> AMEIE_R {
        AMEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - 192 Block mode This bit defines the number of blocks per line
    #[inline(always)]
    pub fn bm192(&self) -> BM192_R {
        BM192_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Cache enable This bit enables the cache unit.
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Cache lock This bit lock the cache onto the buffer defined in the CLB field.
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Cache lock buffer This field select the buffer on which the cache is locked.
    #[inline(always)]
    pub fn clb(&self) -> CLB_R {
        CLB_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Force caching This bit force the caching into the cache regardless of the MPU attributes. The cache must be enable (CE bit set).
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Prefetch disable This bit disables the prefetch of the cache.
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Outter cachability This bit configure the cachability of an access generated by the GFXMMU cache.
    #[inline(always)]
    pub fn oc(&self) -> OC_R {
        OC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Outter bufferability This bit configure the bufferability of an access generated by the GFXMMU cache.
    #[inline(always)]
    pub fn ob(&self) -> OB_R {
        OB_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("b0oie", &self.b0oie())
            .field("b1oie", &self.b1oie())
            .field("b2oie", &self.b2oie())
            .field("b3oie", &self.b3oie())
            .field("ameie", &self.ameie())
            .field("bm192", &self.bm192())
            .field("ce", &self.ce())
            .field("cl", &self.cl())
            .field("clb", &self.clb())
            .field("fc", &self.fc())
            .field("pd", &self.pd())
            .field("oc", &self.oc())
            .field("ob", &self.ob())
            .finish()
    }
}
impl W {
    ///Bit 0 - Buffer 0 overflow interrupt enable This bit enables the buffer 0 overflow interrupt.
    #[inline(always)]
    pub fn b0oie(&mut self) -> B0OIE_W<CRrs> {
        B0OIE_W::new(self, 0)
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable This bit enables the buffer 1 overflow interrupt.
    #[inline(always)]
    pub fn b1oie(&mut self) -> B1OIE_W<CRrs> {
        B1OIE_W::new(self, 1)
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable This bit enables the buffer 2 overflow interrupt.
    #[inline(always)]
    pub fn b2oie(&mut self) -> B2OIE_W<CRrs> {
        B2OIE_W::new(self, 2)
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable This bit enables the buffer 3 overflow interrupt.
    #[inline(always)]
    pub fn b3oie(&mut self) -> B3OIE_W<CRrs> {
        B3OIE_W::new(self, 3)
    }
    ///Bit 4 - AHB master error interrupt enable This bit enables the AHB master error interrupt.
    #[inline(always)]
    pub fn ameie(&mut self) -> AMEIE_W<CRrs> {
        AMEIE_W::new(self, 4)
    }
    ///Bit 6 - 192 Block mode This bit defines the number of blocks per line
    #[inline(always)]
    pub fn bm192(&mut self) -> BM192_W<CRrs> {
        BM192_W::new(self, 6)
    }
    ///Bit 7 - Cache enable This bit enables the cache unit.
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W<CRrs> {
        CE_W::new(self, 7)
    }
    ///Bit 8 - Cache lock This bit lock the cache onto the buffer defined in the CLB field.
    #[inline(always)]
    pub fn cl(&mut self) -> CL_W<CRrs> {
        CL_W::new(self, 8)
    }
    ///Bits 9:10 - Cache lock buffer This field select the buffer on which the cache is locked.
    #[inline(always)]
    pub fn clb(&mut self) -> CLB_W<CRrs> {
        CLB_W::new(self, 9)
    }
    ///Bit 11 - Force caching This bit force the caching into the cache regardless of the MPU attributes. The cache must be enable (CE bit set).
    #[inline(always)]
    pub fn fc(&mut self) -> FC_W<CRrs> {
        FC_W::new(self, 11)
    }
    ///Bit 12 - Prefetch disable This bit disables the prefetch of the cache.
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<CRrs> {
        PD_W::new(self, 12)
    }
    ///Bit 16 - Outter cachability This bit configure the cachability of an access generated by the GFXMMU cache.
    #[inline(always)]
    pub fn oc(&mut self) -> OC_W<CRrs> {
        OC_W::new(self, 16)
    }
    ///Bit 17 - Outter bufferability This bit configure the bufferability of an access generated by the GFXMMU cache.
    #[inline(always)]
    pub fn ob(&mut self) -> OB_W<CRrs> {
        OB_W::new(self, 17)
    }
}
/**GFXMMU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GFXMMU:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}