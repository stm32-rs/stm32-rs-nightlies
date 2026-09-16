///Register `VDDIO4CCCR` reader
pub type R = crate::R<VDDIO4CCCRrs>;
///Register `VDDIO4CCCR` writer
pub type W = crate::W<VDDIO4CCCRrs>;
///Field `RANSRC` reader - These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.
pub type RANSRC_R = crate::FieldReader;
///Field `RANSRC` writer - These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.
pub type RANSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RAPSRC` reader - These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.
pub type RAPSRC_R = crate::FieldReader;
///Field `RAPSRC` writer - These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.
pub type RAPSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EN` reader - Enables the compensation cell of I/Os supplied by VDDIOx.
pub type EN_R = crate::BitReader;
///Field `EN` writer - Enables the compensation cell of I/Os supplied by VDDIOx.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS` reader - Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.
pub type CS_R = crate::BitReader;
///Field `CS` writer - Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.
pub type CS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.
    #[inline(always)]
    pub fn ransrc(&self) -> RANSRC_R {
        RANSRC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.
    #[inline(always)]
    pub fn rapsrc(&self) -> RAPSRC_R {
        RAPSRC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Enables the compensation cell of I/Os supplied by VDDIOx.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDDIO4CCCR")
            .field("ransrc", &self.ransrc())
            .field("rapsrc", &self.rapsrc())
            .field("en", &self.en())
            .field("cs", &self.cs())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.
    #[inline(always)]
    pub fn ransrc(&mut self) -> RANSRC_W<'_, VDDIO4CCCRrs> {
        RANSRC_W::new(self, 0)
    }
    ///Bits 4:7 - These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.
    #[inline(always)]
    pub fn rapsrc(&mut self) -> RAPSRC_W<'_, VDDIO4CCCRrs> {
        RAPSRC_W::new(self, 4)
    }
    ///Bit 8 - Enables the compensation cell of I/Os supplied by VDDIOx.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, VDDIO4CCCRrs> {
        EN_W::new(self, 8)
    }
    ///Bit 9 - Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<'_, VDDIO4CCCRrs> {
        CS_W::new(self, 9)
    }
}
/**SYSCFG VDDIO4 compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`vddio4cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddio4cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SYSCFG:VDDIO4CCCR)*/
pub struct VDDIO4CCCRrs;
impl crate::RegisterSpec for VDDIO4CCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vddio4cccr::R`](R) reader structure
impl crate::Readable for VDDIO4CCCRrs {}
///`write(|w| ..)` method takes [`vddio4cccr::W`](W) writer structure
impl crate::Writable for VDDIO4CCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VDDIO4CCCR to value 0
impl crate::Resettable for VDDIO4CCCRrs {}
