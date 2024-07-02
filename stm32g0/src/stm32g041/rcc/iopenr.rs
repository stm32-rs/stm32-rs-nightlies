///Register `IOPENR` reader
pub type R = crate::R<IOPENRrs>;
///Register `IOPENR` writer
pub type W = crate::W<IOPENRrs>;
///Field `IOPAEN` reader - I/O port A clock enable
pub type IOPAEN_R = crate::BitReader;
///Field `IOPAEN` writer - I/O port A clock enable
pub type IOPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOPBEN` reader - I/O port B clock enable
pub type IOPBEN_R = crate::BitReader;
///Field `IOPBEN` writer - I/O port B clock enable
pub type IOPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOPCEN` reader - I/O port C clock enable
pub type IOPCEN_R = crate::BitReader;
///Field `IOPCEN` writer - I/O port C clock enable
pub type IOPCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOPDEN` reader - I/O port D clock enable
pub type IOPDEN_R = crate::BitReader;
///Field `IOPDEN` writer - I/O port D clock enable
pub type IOPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOPFEN` reader - I/O port F clock enable
pub type IOPFEN_R = crate::BitReader;
///Field `IOPFEN` writer - I/O port F clock enable
pub type IOPFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPENR")
            .field("iopaen", &self.iopaen())
            .field("iopben", &self.iopben())
            .field("iopcen", &self.iopcen())
            .field("iopden", &self.iopden())
            .field("iopfen", &self.iopfen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<IOPENRrs> {
        IOPAEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<IOPENRrs> {
        IOPBEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<IOPENRrs> {
        IOPCEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<IOPENRrs> {
        IOPDEN_W::new(self, 3)
    }
    ///Bit 5 - I/O port F clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopfen(&mut self) -> IOPFEN_W<IOPENRrs> {
        IOPFEN_W::new(self, 5)
    }
}
/**GPIO clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#RCC:IOPENR)*/
pub struct IOPENRrs;
impl crate::RegisterSpec for IOPENRrs {
    type Ux = u32;
}
///`read()` method returns [`iopenr::R`](R) reader structure
impl crate::Readable for IOPENRrs {}
///`write(|w| ..)` method takes [`iopenr::W`](W) writer structure
impl crate::Writable for IOPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IOPENR to value 0
impl crate::Resettable for IOPENRrs {
    const RESET_VALUE: u32 = 0;
}
