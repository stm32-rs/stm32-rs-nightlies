///Register `DMAC0CR` reader
pub type R = crate::R<DMAC0CRrs>;
///Register `DMAC0CR` writer
pub type W = crate::W<DMAC0CRrs>;
///Field `MSS` reader - MSS
pub type MSS_R = crate::FieldReader<u16>;
///Field `MSS` writer - MSS
pub type MSS_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `PBLX8` reader - PBLX8
pub type PBLX8_R = crate::BitReader;
///Field `PBLX8` writer - PBLX8
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSL` reader - DSL
pub type DSL_R = crate::FieldReader;
///Field `DSL` writer - DSL
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:13 - MSS
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 16 - PBLX8
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:20 - DSL
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0CR")
            .field("mss", &self.mss())
            .field("pblx8", &self.pblx8())
            .field("dsl", &self.dsl())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - MSS
    #[inline(always)]
    pub fn mss(&mut self) -> MSS_W<DMAC0CRrs> {
        MSS_W::new(self, 0)
    }
    ///Bit 16 - PBLX8
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<DMAC0CRrs> {
        PBLX8_W::new(self, 16)
    }
    ///Bits 18:20 - DSL
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<DMAC0CRrs> {
        DSL_W::new(self, 18)
    }
}
/**Channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`dmac0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAC0CR)*/
pub struct DMAC0CRrs;
impl crate::RegisterSpec for DMAC0CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0cr::R`](R) reader structure
impl crate::Readable for DMAC0CRrs {}
///`write(|w| ..)` method takes [`dmac0cr::W`](W) writer structure
impl crate::Writable for DMAC0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMAC0CR to value 0
impl crate::Resettable for DMAC0CRrs {
    const RESET_VALUE: u32 = 0;
}