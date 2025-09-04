///Register `CGCR` reader
pub type R = crate::R<CGCRrs>;
///Register `CGCR` writer
pub type W = crate::W<CGCRrs>;
///Field `LCS` reader - line clock source This field configures the line clock source.
pub type LCS_R = crate::FieldReader;
///Field `LCS` writer - line clock source This field configures the line clock source.
pub type LCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LCCCS` reader - line clock counter clock source This bit configures the clock source for the line clock counter.
pub type LCCCS_R = crate::BitReader;
///Field `LCCCS` writer - line clock counter clock source This bit configures the clock source for the line clock counter.
pub type LCCCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCCFR` writer - line clock counter force reload This bit forces line clock counter reload.
pub type LCCFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCCHRS` reader - line clock counter hardware reload source This field configures the hardware reload source for the line clock counter.
pub type LCCHRS_R = crate::FieldReader;
///Field `LCCHRS` writer - line clock counter hardware reload source This field configures the hardware reload source for the line clock counter.
pub type LCCHRS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FCS` reader - frame clock source This field configures the frame clock source
pub type FCS_R = crate::FieldReader;
///Field `FCS` writer - frame clock source This field configures the frame clock source
pub type FCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FCCCS` reader - frame clock counter clock source This field configures the clock source for the frame clock counter.
pub type FCCCS_R = crate::FieldReader;
///Field `FCCCS` writer - frame clock counter clock source This field configures the clock source for the frame clock counter.
pub type FCCCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FCCFR` writer - frame clock counter force reload This bit forces frame clock counter reload
pub type FCCFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCCHRS` reader - frame- -clock counter hardware reload source This field configures the hardware reload source for the frame- -clock counter.
pub type FCCHRS_R = crate::FieldReader;
///Field `FCCHRS` writer - frame- -clock counter hardware reload source This field configures the hardware reload source for the frame- -clock counter.
pub type FCCHRS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - line clock source This field configures the line clock source.
    #[inline(always)]
    pub fn lcs(&self) -> LCS_R {
        LCS_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - line clock counter clock source This bit configures the clock source for the line clock counter.
    #[inline(always)]
    pub fn lcccs(&self) -> LCCCS_R {
        LCCCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 12:14 - line clock counter hardware reload source This field configures the hardware reload source for the line clock counter.
    #[inline(always)]
    pub fn lcchrs(&self) -> LCCHRS_R {
        LCCHRS_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - frame clock source This field configures the frame clock source
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - frame clock counter clock source This field configures the clock source for the frame clock counter.
    #[inline(always)]
    pub fn fcccs(&self) -> FCCCS_R {
        FCCCS_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 28:30 - frame- -clock counter hardware reload source This field configures the hardware reload source for the frame- -clock counter.
    #[inline(always)]
    pub fn fcchrs(&self) -> FCCHRS_R {
        FCCHRS_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CGCR")
            .field("lcs", &self.lcs())
            .field("lcccs", &self.lcccs())
            .field("lcchrs", &self.lcchrs())
            .field("fcs", &self.fcs())
            .field("fcccs", &self.fcccs())
            .field("fcchrs", &self.fcchrs())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - line clock source This field configures the line clock source.
    #[inline(always)]
    pub fn lcs(&mut self) -> LCS_W<CGCRrs> {
        LCS_W::new(self, 0)
    }
    ///Bit 4 - line clock counter clock source This bit configures the clock source for the line clock counter.
    #[inline(always)]
    pub fn lcccs(&mut self) -> LCCCS_W<CGCRrs> {
        LCCCS_W::new(self, 4)
    }
    ///Bit 8 - line clock counter force reload This bit forces line clock counter reload.
    #[inline(always)]
    pub fn lccfr(&mut self) -> LCCFR_W<CGCRrs> {
        LCCFR_W::new(self, 8)
    }
    ///Bits 12:14 - line clock counter hardware reload source This field configures the hardware reload source for the line clock counter.
    #[inline(always)]
    pub fn lcchrs(&mut self) -> LCCHRS_W<CGCRrs> {
        LCCHRS_W::new(self, 12)
    }
    ///Bits 16:18 - frame clock source This field configures the frame clock source
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W<CGCRrs> {
        FCS_W::new(self, 16)
    }
    ///Bits 20:22 - frame clock counter clock source This field configures the clock source for the frame clock counter.
    #[inline(always)]
    pub fn fcccs(&mut self) -> FCCCS_W<CGCRrs> {
        FCCCS_W::new(self, 20)
    }
    ///Bit 24 - frame clock counter force reload This bit forces frame clock counter reload
    #[inline(always)]
    pub fn fccfr(&mut self) -> FCCFR_W<CGCRrs> {
        FCCFR_W::new(self, 24)
    }
    ///Bits 28:30 - frame- -clock counter hardware reload source This field configures the hardware reload source for the frame- -clock counter.
    #[inline(always)]
    pub fn fcchrs(&mut self) -> FCCHRS_W<CGCRrs> {
        FCCHRS_W::new(self, 28)
    }
}
/**GFXTIM clock generator configuration register

You can [`read`](crate::Reg::read) this register and get [`cgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#GFXTIM:CGCR)*/
pub struct CGCRrs;
impl crate::RegisterSpec for CGCRrs {
    type Ux = u32;
}
///`read()` method returns [`cgcr::R`](R) reader structure
impl crate::Readable for CGCRrs {}
///`write(|w| ..)` method takes [`cgcr::W`](W) writer structure
impl crate::Writable for CGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CGCR to value 0
impl crate::Resettable for CGCRrs {}
