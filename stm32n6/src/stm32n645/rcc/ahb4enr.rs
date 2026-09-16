///Register `AHB4ENR` reader
pub type R = crate::R<AHB4ENRrs>;
///Register `AHB4ENR` writer
pub type W = crate::W<AHB4ENRrs>;
///Field `GPIOAEN` reader - GPIOA enable
pub type GPIOAEN_R = crate::BitReader;
///Field `GPIOAEN` writer - GPIOA enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBEN` reader - GPIOB enable
pub type GPIOBEN_R = crate::BitReader;
///Field `GPIOBEN` writer - GPIOB enable
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCEN` reader - GPIOC enable
pub type GPIOCEN_R = crate::BitReader;
///Field `GPIOCEN` writer - GPIOC enable
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODEN` reader - GPIOD enable
pub type GPIODEN_R = crate::BitReader;
///Field `GPIODEN` writer - GPIOD enable
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOEEN` reader - GPIOE enable
pub type GPIOEEN_R = crate::BitReader;
///Field `GPIOEEN` writer - GPIOE enable
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFEN` reader - GPIOF enable
pub type GPIOFEN_R = crate::BitReader;
///Field `GPIOFEN` writer - GPIOF enable
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGEN` reader - GPIOG enable
pub type GPIOGEN_R = crate::BitReader;
///Field `GPIOGEN` writer - GPIOG enable
pub type GPIOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHEN` reader - GPIOH enable
pub type GPIOHEN_R = crate::BitReader;
///Field `GPIOHEN` writer - GPIOH enable
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONEN` reader - GPION enable
pub type GPIONEN_R = crate::BitReader;
///Field `GPIONEN` writer - GPION enable
pub type GPIONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOOEN` reader - GPIOO enable
pub type GPIOOEN_R = crate::BitReader;
///Field `GPIOOEN` writer - GPIOO enable
pub type GPIOOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPEN` reader - GPIOP enable
pub type GPIOPEN_R = crate::BitReader;
///Field `GPIOPEN` writer - GPIOP enable
pub type GPIOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQEN` reader - GPIOQ enable
pub type GPIOQEN_R = crate::BitReader;
///Field `GPIOQEN` writer - GPIOQ enable
pub type GPIOQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWREN` reader - PWR enable
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - PWR enable
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC enable
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOA enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - GPION enable
    #[inline(always)]
    pub fn gpionen(&self) -> GPIONEN_R {
        GPIONEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GPIOO enable
    #[inline(always)]
    pub fn gpiooen(&self) -> GPIOOEN_R {
        GPIOOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GPIOP enable
    #[inline(always)]
    pub fn gpiopen(&self) -> GPIOPEN_R {
        GPIOPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - GPIOQ enable
    #[inline(always)]
    pub fn gpioqen(&self) -> GPIOQEN_R {
        GPIOQEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - PWR enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CRC enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpionen", &self.gpionen())
            .field("gpiooen", &self.gpiooen())
            .field("gpiopen", &self.gpiopen())
            .field("gpioqen", &self.gpioqen())
            .field("pwren", &self.pwren())
            .field("crcen", &self.crcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB4ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOB enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB4ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOC enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB4ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOD enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB4ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOE enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB4ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOF enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB4ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOG enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB4ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOH enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB4ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 13 - GPION enable
    #[inline(always)]
    pub fn gpionen(&mut self) -> GPIONEN_W<'_, AHB4ENRrs> {
        GPIONEN_W::new(self, 13)
    }
    ///Bit 14 - GPIOO enable
    #[inline(always)]
    pub fn gpiooen(&mut self) -> GPIOOEN_W<'_, AHB4ENRrs> {
        GPIOOEN_W::new(self, 14)
    }
    ///Bit 15 - GPIOP enable
    #[inline(always)]
    pub fn gpiopen(&mut self) -> GPIOPEN_W<'_, AHB4ENRrs> {
        GPIOPEN_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ enable
    #[inline(always)]
    pub fn gpioqen(&mut self) -> GPIOQEN_W<'_, AHB4ENRrs> {
        GPIOQEN_W::new(self, 16)
    }
    ///Bit 18 - PWR enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, AHB4ENRrs> {
        PWREN_W::new(self, 18)
    }
    ///Bit 19 - CRC enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB4ENRrs> {
        CRCEN_W::new(self, 19)
    }
}
/**RCC AHB4 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB4ENR)*/
pub struct AHB4ENRrs;
impl crate::RegisterSpec for AHB4ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4enr::R`](R) reader structure
impl crate::Readable for AHB4ENRrs {}
///`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure
impl crate::Writable for AHB4ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4ENR to value 0x0004_0000
impl crate::Resettable for AHB4ENRrs {
    const RESET_VALUE: u32 = 0x0004_0000;
}
