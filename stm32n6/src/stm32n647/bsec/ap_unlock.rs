///Register `AP_UNLOCK` reader
pub type R = crate::R<AP_UNLOCKrs>;
///Register `AP_UNLOCK` writer
pub type W = crate::W<AP_UNLOCKrs>;
///Field `UNLOCK` reader - any other value: do not unlock
pub type UNLOCK_R = crate::FieldReader;
///Field `UNLOCK` writer - any other value: do not unlock
pub type UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - any other value: do not unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AP_UNLOCK")
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - any other value: do not unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<'_, AP_UNLOCKrs> {
        UNLOCK_W::new(self, 0)
    }
}
/**BSEC AP Unlock

You can [`read`](crate::Reg::read) this register and get [`ap_unlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap_unlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:AP_UNLOCK)*/
pub struct AP_UNLOCKrs;
impl crate::RegisterSpec for AP_UNLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`ap_unlock::R`](R) reader structure
impl crate::Readable for AP_UNLOCKrs {}
///`write(|w| ..)` method takes [`ap_unlock::W`](W) writer structure
impl crate::Writable for AP_UNLOCKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AP_UNLOCK to value 0
impl crate::Resettable for AP_UNLOCKrs {}
