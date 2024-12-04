///Register `HSPI_DCR2` reader
pub type R = crate::R<HSPI_DCR2rs>;
///Register `HSPI_DCR2` writer
pub type W = crate::W<HSPI_DCR2rs>;
///Field `PRESCALER` reader - Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (valueÂ +Â 1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50Â %. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case HSPI_CALOSR or HSPI_CALISR have been written in the meantime. BUSY stays high during the whole calibration execution.
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (valueÂ +Â 1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50Â %. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case HSPI_CALOSR or HSPI_CALISR have been written in the meantime. BUSY stays high during the whole calibration execution.
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WRAPSIZE` reader - Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the HSPI_WPIR register. 110-111: Reserved
pub type WRAPSIZE_R = crate::FieldReader;
///Field `WRAPSIZE` writer - Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the HSPI_WPIR register. 110-111: Reserved
pub type WRAPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:7 - Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (valueÂ +Â 1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50Â %. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case HSPI_CALOSR or HSPI_CALISR have been written in the meantime. BUSY stays high during the whole calibration execution.
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:18 - Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the HSPI_WPIR register. 110-111: Reserved
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_DCR2")
            .field("prescaler", &self.prescaler())
            .field("wrapsize", &self.wrapsize())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (valueÂ +Â 1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50Â %. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case HSPI_CALOSR or HSPI_CALISR have been written in the meantime. BUSY stays high during the whole calibration execution.
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<HSPI_DCR2rs> {
        PRESCALER_W::new(self, 0)
    }
    ///Bits 16:18 - Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the HSPI_WPIR register. 110-111: Reserved
    #[inline(always)]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W<HSPI_DCR2rs> {
        WRAPSIZE_W::new(self, 16)
    }
}
/**HSPI device configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hspi_dcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_dcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:HSPI_DCR2)*/
pub struct HSPI_DCR2rs;
impl crate::RegisterSpec for HSPI_DCR2rs {
    type Ux = u32;
}
///`read()` method returns [`hspi_dcr2::R`](R) reader structure
impl crate::Readable for HSPI_DCR2rs {}
///`write(|w| ..)` method takes [`hspi_dcr2::W`](W) writer structure
impl crate::Writable for HSPI_DCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_DCR2 to value 0
impl crate::Resettable for HSPI_DCR2rs {
    const RESET_VALUE: u32 = 0;
}
