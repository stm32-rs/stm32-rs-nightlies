///Register `RCC_IOPSMENR` reader
pub type R = crate::R<RCC_IOPSMENRrs>;
///Register `RCC_IOPSMENR` writer
pub type W = crate::W<RCC_IOPSMENRrs>;
///Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode Set and cleared by software.
pub type GPIOASMEN_R = crate::BitReader;
///Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode Set and cleared by software.
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode Set and cleared by software.
pub type GPIOBSMEN_R = crate::BitReader;
///Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode Set and cleared by software.
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode Set and cleared by software.
pub type GPIOCSMEN_R = crate::BitReader;
///Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode Set and cleared by software.
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type GPIODSMEN_R = crate::BitReader;
///Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type GPIODSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOESMEN` reader - I/O port E clock enable during Sleep mode Set and cleared by software.
pub type GPIOESMEN_R = crate::BitReader;
///Field `GPIOESMEN` writer - I/O port E clock enable during Sleep mode Set and cleared by software.
pub type GPIOESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode Set and cleared by software.
pub type GPIOFSMEN_R = crate::BitReader;
///Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode Set and cleared by software.
pub type GPIOFSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IOPSMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiofsmen", &self.gpiofsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<RCC_IOPSMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<RCC_IOPSMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<RCC_IOPSMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<RCC_IOPSMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<RCC_IOPSMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<RCC_IOPSMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
}
/**I/O port in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_IOPSMENR)*/
pub struct RCC_IOPSMENRrs;
impl crate::RegisterSpec for RCC_IOPSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_iopsmenr::R`](R) reader structure
impl crate::Readable for RCC_IOPSMENRrs {}
///`write(|w| ..)` method takes [`rcc_iopsmenr::W`](W) writer structure
impl crate::Writable for RCC_IOPSMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_IOPSMENR to value 0x3f
impl crate::Resettable for RCC_IOPSMENRrs {
    const RESET_VALUE: u32 = 0x3f;
}