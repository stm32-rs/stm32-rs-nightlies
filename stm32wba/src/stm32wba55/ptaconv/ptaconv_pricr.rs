///Register `PTACONV_PRICR` reader
pub type R = crate::R<PTACONV_PRICRrs>;
///Register `PTACONV_PRICR` writer
pub type W = crate::W<PTACONV_PRICRrs>;
///Field `TPRIORITY` reader - Priority valid time in us.
pub type TPRIORITY_R = crate::FieldReader;
///Field `TPRIORITY` writer - Priority valid time in us.
pub type TPRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PRIPOL` reader - Priority polarity.
pub type PRIPOL_R = crate::BitReader;
///Field `PRIPOL` writer - Priority polarity.
pub type PRIPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Priority valid time in us.
    #[inline(always)]
    pub fn tpriority(&self) -> TPRIORITY_R {
        TPRIORITY_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 15 - Priority polarity.
    #[inline(always)]
    pub fn pripol(&self) -> PRIPOL_R {
        PRIPOL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTACONV_PRICR")
            .field("tpriority", &self.tpriority())
            .field("pripol", &self.pripol())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Priority valid time in us.
    #[inline(always)]
    pub fn tpriority(&mut self) -> TPRIORITY_W<'_, PTACONV_PRICRrs> {
        TPRIORITY_W::new(self, 0)
    }
    ///Bit 15 - Priority polarity.
    #[inline(always)]
    pub fn pripol(&mut self) -> PRIPOL_W<'_, PTACONV_PRICRrs> {
        PRIPOL_W::new(self, 15)
    }
}
/**PTACONV priority control register

You can [`read`](crate::Reg::read) this register and get [`ptaconv_pricr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptaconv_pricr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PTACONV:PTACONV_PRICR)*/
pub struct PTACONV_PRICRrs;
impl crate::RegisterSpec for PTACONV_PRICRrs {
    type Ux = u32;
}
///`read()` method returns [`ptaconv_pricr::R`](R) reader structure
impl crate::Readable for PTACONV_PRICRrs {}
///`write(|w| ..)` method takes [`ptaconv_pricr::W`](W) writer structure
impl crate::Writable for PTACONV_PRICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTACONV_PRICR to value 0x0a
impl crate::Resettable for PTACONV_PRICRrs {
    const RESET_VALUE: u32 = 0x0a;
}
