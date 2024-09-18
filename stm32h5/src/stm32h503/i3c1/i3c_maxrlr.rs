///Register `I3C_MAXRLR` reader
pub type R = crate::R<I3C_MAXRLRrs>;
///Register `I3C_MAXRLR` writer
pub type W = crate::W<I3C_MAXRLRrs>;
///Field `MRL` reader - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
pub type MRL_R = crate::FieldReader<u16>;
///Field `MRL` writer - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
pub type MRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `IBIP` reader - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
pub type IBIP_R = crate::FieldReader;
///Field `IBIP` writer - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
pub type IBIP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
    #[inline(always)]
    pub fn mrl(&self) -> MRL_R {
        MRL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
    #[inline(always)]
    pub fn ibip(&self) -> IBIP_R {
        IBIP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MAXRLR")
            .field("mrl", &self.mrl())
            .field("ibip", &self.ibip())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
    #[inline(always)]
    #[must_use]
    pub fn mrl(&mut self) -> MRL_W<I3C_MAXRLRrs> {
        MRL_W::new(self, 0)
    }
    ///Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
    #[inline(always)]
    #[must_use]
    pub fn ibip(&mut self) -> IBIP_W<I3C_MAXRLRrs> {
        IBIP_W::new(self, 16)
    }
}
/**I3C maximum read length register

You can [`read`](crate::Reg::read) this register and get [`i3c_maxrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_maxrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_MAXRLR)*/
pub struct I3C_MAXRLRrs;
impl crate::RegisterSpec for I3C_MAXRLRrs {
    type Ux = u32;
}
///`read()` method returns [`i3c_maxrlr::R`](R) reader structure
impl crate::Readable for I3C_MAXRLRrs {}
///`write(|w| ..)` method takes [`i3c_maxrlr::W`](W) writer structure
impl crate::Writable for I3C_MAXRLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_MAXRLR to value 0
impl crate::Resettable for I3C_MAXRLRrs {
    const RESET_VALUE: u32 = 0;
}
