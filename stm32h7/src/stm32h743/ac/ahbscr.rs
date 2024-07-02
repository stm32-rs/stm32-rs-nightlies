///Register `AHBSCR` reader
pub type R = crate::R<AHBSCRrs>;
///Register `AHBSCR` writer
pub type W = crate::W<AHBSCRrs>;
///Field `CTL` reader - CTL
pub type CTL_R = crate::FieldReader;
///Field `CTL` writer - CTL
pub type CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TPRI` reader - TPRI
pub type TPRI_R = crate::FieldReader<u16>;
///Field `TPRI` writer - TPRI
pub type TPRI_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `INITCOUNT` reader - INITCOUNT
pub type INITCOUNT_R = crate::FieldReader;
///Field `INITCOUNT` writer - INITCOUNT
pub type INITCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - CTL
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:10 - TPRI
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bits 11:15 - INITCOUNT
    #[inline(always)]
    pub fn initcount(&self) -> INITCOUNT_R {
        INITCOUNT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSCR")
            .field("ctl", &self.ctl())
            .field("tpri", &self.tpri())
            .field("initcount", &self.initcount())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CTL
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CTL_W<AHBSCRrs> {
        CTL_W::new(self, 0)
    }
    ///Bits 2:10 - TPRI
    #[inline(always)]
    #[must_use]
    pub fn tpri(&mut self) -> TPRI_W<AHBSCRrs> {
        TPRI_W::new(self, 2)
    }
    ///Bits 11:15 - INITCOUNT
    #[inline(always)]
    #[must_use]
    pub fn initcount(&mut self) -> INITCOUNT_W<AHBSCRrs> {
        INITCOUNT_W::new(self, 11)
    }
}
/**AHB Slave Control register

You can [`read`](crate::Reg::read) this register and get [`ahbscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#AC:AHBSCR)*/
pub struct AHBSCRrs;
impl crate::RegisterSpec for AHBSCRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbscr::R`](R) reader structure
impl crate::Readable for AHBSCRrs {}
///`write(|w| ..)` method takes [`ahbscr::W`](W) writer structure
impl crate::Writable for AHBSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHBSCR to value 0
impl crate::Resettable for AHBSCRrs {
    const RESET_VALUE: u32 = 0;
}
