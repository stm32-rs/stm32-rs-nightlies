///Register `I3C_MAXWLR` reader
pub type R = crate::R<I3C_MAXWLRrs>;
///Register `I3C_MAXWLR` writer
pub type W = crate::W<I3C_MAXWLRrs>;
///Field `MWL` reader - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC.
pub type MWL_R = crate::FieldReader<u16>;
///Field `MWL` writer - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC.
pub type MWL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC.
    #[inline(always)]
    pub fn mwl(&self) -> MWL_R {
        MWL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MAXWLR")
            .field("mwl", &self.mwl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC.
    #[inline(always)]
    #[must_use]
    pub fn mwl(&mut self) -> MWL_W<I3C_MAXWLRrs> {
        MWL_W::new(self, 0)
    }
}
/**I3C maximum write length register

You can [`read`](crate::Reg::read) this register and get [`i3c_maxwlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_maxwlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_MAXWLR)*/
pub struct I3C_MAXWLRrs;
impl crate::RegisterSpec for I3C_MAXWLRrs {
    type Ux = u32;
}
///`read()` method returns [`i3c_maxwlr::R`](R) reader structure
impl crate::Readable for I3C_MAXWLRrs {}
///`write(|w| ..)` method takes [`i3c_maxwlr::W`](W) writer structure
impl crate::Writable for I3C_MAXWLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_MAXWLR to value 0
impl crate::Resettable for I3C_MAXWLRrs {
    const RESET_VALUE: u32 = 0;
}
