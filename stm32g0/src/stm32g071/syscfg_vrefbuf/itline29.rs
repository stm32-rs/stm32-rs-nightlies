///Register `ITLINE29` reader
pub type R = crate::R<ITLINE29rs>;
///Field `USART3` reader - USART3
pub type USART3_R = crate::BitReader;
///Field `USART4` reader - USART4
pub type USART4_R = crate::BitReader;
///Field `USART5` reader - USART5
pub type USART5_R = crate::BitReader;
impl R {
    ///Bit 0 - USART3
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART4
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - USART5
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE29")
            .field("usart3", &self.usart3())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .finish()
    }
}
/**interrupt line 29 status register

You can [`read`](crate::Reg::read) this register and get [`itline29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SYSCFG_VREFBUF:ITLINE29)*/
pub struct ITLINE29rs;
impl crate::RegisterSpec for ITLINE29rs {
    type Ux = u32;
}
///`read()` method returns [`itline29::R`](R) reader structure
impl crate::Readable for ITLINE29rs {}
///`reset()` method sets ITLINE29 to value 0
impl crate::Resettable for ITLINE29rs {
    const RESET_VALUE: u32 = 0;
}
