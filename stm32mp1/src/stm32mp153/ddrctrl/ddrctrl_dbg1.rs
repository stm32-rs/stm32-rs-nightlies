///Register `DDRCTRL_DBG1` reader
pub type R = crate::R<DDRCTRL_DBG1rs>;
///Register `DDRCTRL_DBG1` writer
pub type W = crate::W<DDRCTRL_DBG1rs>;
///Field `DIS_DQ` reader - DIS_DQ
pub type DIS_DQ_R = crate::BitReader;
///Field `DIS_DQ` writer - DIS_DQ
pub type DIS_DQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_HIF` reader - DIS_HIF
pub type DIS_HIF_R = crate::BitReader;
///Field `DIS_HIF` writer - DIS_HIF
pub type DIS_HIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DIS_DQ
    #[inline(always)]
    pub fn dis_dq(&self) -> DIS_DQ_R {
        DIS_DQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DIS_HIF
    #[inline(always)]
    pub fn dis_hif(&self) -> DIS_HIF_R {
        DIS_HIF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_DBG1")
            .field("dis_dq", &self.dis_dq())
            .field("dis_hif", &self.dis_hif())
            .finish()
    }
}
impl W {
    ///Bit 0 - DIS_DQ
    #[inline(always)]
    #[must_use]
    pub fn dis_dq(&mut self) -> DIS_DQ_W<DDRCTRL_DBG1rs> {
        DIS_DQ_W::new(self, 0)
    }
    ///Bit 1 - DIS_HIF
    #[inline(always)]
    #[must_use]
    pub fn dis_hif(&mut self) -> DIS_HIF_W<DDRCTRL_DBG1rs> {
        DIS_HIF_W::new(self, 1)
    }
}
/**DDRCTRL debug register 1

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_dbg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_dbg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_DBG1)*/
pub struct DDRCTRL_DBG1rs;
impl crate::RegisterSpec for DDRCTRL_DBG1rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_dbg1::R`](R) reader structure
impl crate::Readable for DDRCTRL_DBG1rs {}
///`write(|w| ..)` method takes [`ddrctrl_dbg1::W`](W) writer structure
impl crate::Writable for DDRCTRL_DBG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_DBG1 to value 0
impl crate::Resettable for DDRCTRL_DBG1rs {
    const RESET_VALUE: u32 = 0;
}
