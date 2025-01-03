///Register `IOPRSTR` reader
pub type R = crate::R<IOPRSTRrs>;
///Register `IOPRSTR` writer
pub type W = crate::W<IOPRSTRrs>;
///Field `GPIOARST` reader - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_R = crate::BitReader;
///Field `GPIOARST` writer - I/O port A reset This bit is set and cleared by software.
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRST` reader - I/O port B reset This bit is set and cleared by software.
pub type GPIOBRST_R = crate::BitReader;
///Field `GPIOBRST` writer - I/O port B reset This bit is set and cleared by software.
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCRST` reader - I/O port C reset This bit is set and cleared by software.
pub type GPIOCRST_R = crate::BitReader;
///Field `GPIOCRST` writer - I/O port C reset This bit is set and cleared by software.
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODRST` reader - I/O port D reset This bit is set and cleared by software.
pub type GPIODRST_R = crate::BitReader;
///Field `GPIODRST` writer - I/O port D reset This bit is set and cleared by software.
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFRST` reader - I/O port F reset This bit is set and cleared by software.
pub type GPIOFRST_R = crate::BitReader;
///Field `GPIOFRST` writer - I/O port F reset This bit is set and cleared by software.
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPRSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpiofrst", &self.gpiofrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<IOPRSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - I/O port B reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<IOPRSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - I/O port C reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<IOPRSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - I/O port D reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<IOPRSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 5 - I/O port F reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<IOPRSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
}
/**RCC I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`ioprstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:IOPRSTR)*/
pub struct IOPRSTRrs;
impl crate::RegisterSpec for IOPRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ioprstr::R`](R) reader structure
impl crate::Readable for IOPRSTRrs {}
///`write(|w| ..)` method takes [`ioprstr::W`](W) writer structure
impl crate::Writable for IOPRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IOPRSTR to value 0
impl crate::Resettable for IOPRSTRrs {
    const RESET_VALUE: u32 = 0;
}
