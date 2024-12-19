///Register `SYSCFG_SCSR` reader
pub type R = crate::R<SYSCFG_SCSRrs>;
///Register `SYSCFG_SCSR` writer
pub type W = crate::W<SYSCFG_SCSRrs>;
///Field `SRAM2ER` reader - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register.
pub type SRAM2ER_R = crate::BitReader;
///Field `SRAM2ER` writer - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register.
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2BSY` reader - SRAM2 busy by erase operation
pub type SRAM2BSY_R = crate::BitReader;
impl R {
    ///Bit 0 - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register.
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 busy by erase operation
    #[inline(always)]
    pub fn sram2bsy(&self) -> SRAM2BSY_R {
        SRAM2BSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_SCSR")
            .field("sram2er", &self.sram2er())
            .field("sram2bsy", &self.sram2bsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register.
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W<SYSCFG_SCSRrs> {
        SRAM2ER_W::new(self, 0)
    }
}
/**SYSCFG SRAM2 control and status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:SYSCFG_SCSR)*/
pub struct SYSCFG_SCSRrs;
impl crate::RegisterSpec for SYSCFG_SCSRrs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_scsr::R`](R) reader structure
impl crate::Readable for SYSCFG_SCSRrs {}
///`write(|w| ..)` method takes [`syscfg_scsr::W`](W) writer structure
impl crate::Writable for SYSCFG_SCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYSCFG_SCSR to value 0
impl crate::Resettable for SYSCFG_SCSRrs {
    const RESET_VALUE: u32 = 0;
}
