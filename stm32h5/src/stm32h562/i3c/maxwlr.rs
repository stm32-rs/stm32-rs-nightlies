#[doc = "Register `MAXWLR` reader"]
pub type R = crate::R<MAXWLRrs>;
#[doc = "Register `MAXWLR` writer"]
pub type W = crate::W<MAXWLRrs>;
#[doc = "Field `MWL` reader - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
pub type MWL_R = crate::FieldReader<u16>;
#[doc = "Field `MWL` writer - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
pub type MWL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
    #[inline(always)]
    pub fn mwl(&self) -> MWL_R {
        MWL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
    #[inline(always)]
    #[must_use]
    pub fn mwl(&mut self) -> MWL_W<MAXWLRrs> {
        MWL_W::new(self, 0)
    }
}
#[doc = "I3C maximum write length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxwlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxwlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXWLRrs;
impl crate::RegisterSpec for MAXWLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxwlr::R`](R) reader structure"]
impl crate::Readable for MAXWLRrs {}
#[doc = "`write(|w| ..)` method takes [`maxwlr::W`](W) writer structure"]
impl crate::Writable for MAXWLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXWLR to value 0"]
impl crate::Resettable for MAXWLRrs {
    const RESET_VALUE: u32 = 0;
}
