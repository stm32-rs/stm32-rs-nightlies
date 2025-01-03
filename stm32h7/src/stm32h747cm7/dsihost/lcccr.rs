///Register `LCCCR` reader
pub type R = crate::R<LCCCRrs>;
///Register `LCCCR` writer
pub type W = crate::W<LCCCRrs>;
///Field `COLC` reader - Color coding
pub type COLC_R = crate::FieldReader;
///Field `COLC` writer - Color coding
pub type COLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LPE` reader - Loosely packed enable
pub type LPE_R = crate::BitReader;
///Field `LPE` writer - Loosely packed enable
pub type LPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Color coding
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Loosely packed enable
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCCCR")
            .field("lpe", &self.lpe())
            .field("colc", &self.colc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Color coding
    #[inline(always)]
    pub fn colc(&mut self) -> COLC_W<LCCCRrs> {
        COLC_W::new(self, 0)
    }
    ///Bit 8 - Loosely packed enable
    #[inline(always)]
    pub fn lpe(&mut self) -> LPE_W<LCCCRrs> {
        LPE_W::new(self, 8)
    }
}
/**DSI Host LTDC current color coding register

You can [`read`](crate::Reg::read) this register and get [`lcccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:LCCCR)*/
pub struct LCCCRrs;
impl crate::RegisterSpec for LCCCRrs {
    type Ux = u32;
}
///`read()` method returns [`lcccr::R`](R) reader structure
impl crate::Readable for LCCCRrs {}
///`write(|w| ..)` method takes [`lcccr::W`](W) writer structure
impl crate::Writable for LCCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCCCR to value 0
impl crate::Resettable for LCCCRrs {
    const RESET_VALUE: u32 = 0;
}
