///Register `RCC_IOPENR` reader
pub type R = crate::R<RCC_IOPENRrs>;
///Register `RCC_IOPENR` writer
pub type W = crate::W<RCC_IOPENRrs>;
///Field `GPIOAEN` reader - I/O port A clock enable This bit is set and cleared by software.
pub type GPIOAEN_R = crate::BitReader;
///Field `GPIOAEN` writer - I/O port A clock enable This bit is set and cleared by software.
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBEN` reader - I/O port B clock enable This bit is set and cleared by software.
pub type GPIOBEN_R = crate::BitReader;
///Field `GPIOBEN` writer - I/O port B clock enable This bit is set and cleared by software.
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCEN` reader - I/O port C clock enable This bit is set and cleared by software.
pub type GPIOCEN_R = crate::BitReader;
///Field `GPIOCEN` writer - I/O port C clock enable This bit is set and cleared by software.
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODEN` reader - I/O port D clock enable This bit is set and cleared by software.
pub type GPIODEN_R = crate::BitReader;
///Field `GPIODEN` writer - I/O port D clock enable This bit is set and cleared by software.
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOEEN` reader - I/O port E clock enable<sup>(1)</sup> This bit is set and cleared by software.
pub type GPIOEEN_R = crate::BitReader;
///Field `GPIOEEN` writer - I/O port E clock enable<sup>(1)</sup> This bit is set and cleared by software.
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFEN` reader - I/O port F clock enable This bit is set and cleared by software.
pub type GPIOFEN_R = crate::BitReader;
///Field `GPIOFEN` writer - I/O port F clock enable This bit is set and cleared by software.
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I/O port A clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable<sup>(1)</sup> This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IOPENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<RCC_IOPENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<RCC_IOPENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<RCC_IOPENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<RCC_IOPENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port E clock enable<sup>(1)</sup> This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<RCC_IOPENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - I/O port F clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<RCC_IOPENRrs> {
        GPIOFEN_W::new(self, 5)
    }
}
/**I/O port clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#RCC:RCC_IOPENR)*/
pub struct RCC_IOPENRrs;
impl crate::RegisterSpec for RCC_IOPENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_iopenr::R`](R) reader structure
impl crate::Readable for RCC_IOPENRrs {}
///`write(|w| ..)` method takes [`rcc_iopenr::W`](W) writer structure
impl crate::Writable for RCC_IOPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_IOPENR to value 0
impl crate::Resettable for RCC_IOPENRrs {
    const RESET_VALUE: u32 = 0;
}