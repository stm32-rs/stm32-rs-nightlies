///Register `WRPAR` reader
pub type R = crate::R<WRPARrs>;
///Register `WRPAR` writer
pub type W = crate::W<WRPARrs>;
///Field `WRPA_PSTRT` reader - WPR area A start page This field contains the first page of the WPR area A. Note that bit 6 is reserved on STM32WBAxEx devices.
pub type WRPA_PSTRT_R = crate::FieldReader;
///Field `WRPA_PSTRT` writer - WPR area A start page This field contains the first page of the WPR area A. Note that bit 6 is reserved on STM32WBAxEx devices.
pub type WRPA_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRPA_PEND` reader - WPR area A end page This field contains the last page of the WPR area A. Note that bit 22 is reserved on STM32WBAxEx devices.
pub type WRPA_PEND_R = crate::FieldReader;
///Field `WRPA_PEND` writer - WPR area A end page This field contains the last page of the WPR area A. Note that bit 22 is reserved on STM32WBAxEx devices.
pub type WRPA_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `UNLOCK` reader - WPR area A unlock
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - WPR area A unlock
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - WPR area A start page This field contains the first page of the WPR area A. Note that bit 6 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpa_pstrt(&self) -> WRPA_PSTRT_R {
        WRPA_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WPR area A end page This field contains the last page of the WPR area A. Note that bit 22 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpa_pend(&self) -> WRPA_PEND_R {
        WRPA_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - WPR area A unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPAR")
            .field("wrpa_pstrt", &self.wrpa_pstrt())
            .field("wrpa_pend", &self.wrpa_pend())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - WPR area A start page This field contains the first page of the WPR area A. Note that bit 6 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpa_pstrt(&mut self) -> WRPA_PSTRT_W<'_, WRPARrs> {
        WRPA_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - WPR area A end page This field contains the last page of the WPR area A. Note that bit 22 is reserved on STM32WBAxEx devices.
    #[inline(always)]
    pub fn wrpa_pend(&mut self) -> WRPA_PEND_W<'_, WRPARrs> {
        WRPA_PEND_W::new(self, 16)
    }
    ///Bit 31 - WPR area A unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<'_, WRPARrs> {
        UNLOCK_W::new(self, 31)
    }
}
/**FLASH WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`wrpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:WRPAR)*/
pub struct WRPARrs;
impl crate::RegisterSpec for WRPARrs {
    type Ux = u32;
}
///`read()` method returns [`wrpar::R`](R) reader structure
impl crate::Readable for WRPARrs {}
///`write(|w| ..)` method takes [`wrpar::W`](W) writer structure
impl crate::Writable for WRPARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPAR to value 0x0f00_ff00
impl crate::Resettable for WRPARrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
