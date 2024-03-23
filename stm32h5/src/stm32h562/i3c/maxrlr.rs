#[doc = "Register `MAXRLR` reader"]
pub type R = crate::R<MAXRLRrs>;
#[doc = "Register `MAXRLR` writer"]
pub type W = crate::W<MAXRLRrs>;
#[doc = "Field `MRL` reader - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
pub type MRL_R = crate::FieldReader<u16>;
#[doc = "Field `MRL` writer - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
pub type MRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IBIP` reader - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
pub type IBIP_R = crate::FieldReader;
#[doc = "Field `IBIP` writer - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
pub type IBIP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
    #[inline(always)]
    pub fn mrl(&self) -> MRL_R {
        MRL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
    #[inline(always)]
    pub fn ibip(&self) -> IBIP_R {
        IBIP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
    #[inline(always)]
    #[must_use]
    pub fn mrl(&mut self) -> MRL_W<MAXRLRrs> {
        MRL_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
    #[inline(always)]
    #[must_use]
    pub fn ibip(&mut self) -> IBIP_W<MAXRLRrs> {
        IBIP_W::new(self, 16)
    }
}
#[doc = "I3C maximum read length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxrlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxrlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXRLRrs;
impl crate::RegisterSpec for MAXRLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxrlr::R`](R) reader structure"]
impl crate::Readable for MAXRLRrs {}
#[doc = "`write(|w| ..)` method takes [`maxrlr::W`](W) writer structure"]
impl crate::Writable for MAXRLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXRLR to value 0"]
impl crate::Resettable for MAXRLRrs {
    const RESET_VALUE: u32 = 0;
}
