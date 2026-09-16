///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
/**Port C configuration I/O pin 13

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Input mode
    Input = 0,
    ///1: General purpose output mode
    Output = 1,
    ///2: Alternate function mode
    Alternate = 2,
    ///3: Analog mode
    Analog = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE13` reader - Port C configuration I/O pin 13
pub type MODE13_R = crate::FieldReader<MODE>;
impl MODE13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Input,
            1 => MODE::Output,
            2 => MODE::Alternate,
            3 => MODE::Analog,
            _ => unreachable!(),
        }
    }
    ///Input mode
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE::Input
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE::Output
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODE::Alternate
    }
    ///Analog mode
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODE::Analog
    }
}
///Field `MODE13` writer - Port C configuration I/O pin 13
pub type MODE13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODE13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Input)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Output)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Alternate)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Analog)
    }
}
///Field `MODE14` reader - Port C configuration I/O pin 14
pub use MODE13_R as MODE14_R;
///Field `MODE15` reader - Port C configuration I/O pin 15 These bits are written by software to configure the I/O mode. Access can be protected by GPIOC SEC15.
pub use MODE13_R as MODE15_R;
///Field `MODE14` writer - Port C configuration I/O pin 14
pub use MODE13_W as MODE14_W;
///Field `MODE15` writer - Port C configuration I/O pin 15 These bits are written by software to configure the I/O mode. Access can be protected by GPIOC SEC15.
pub use MODE13_W as MODE15_W;
impl R {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O mode. Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("mode13", &self.mode13())
            .field("mode14", &self.mode14())
            .field("mode15", &self.mode15())
            .finish()
    }
}
impl W {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W<'_, MODERrs> {
        MODE13_W::new(self, 26)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W<'_, MODERrs> {
        MODE14_W::new(self, 28)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O mode. Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W<'_, MODERrs> {
        MODE15_W::new(self, 30)
    }
}
/**GPIO port C mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MODER to value 0xfc00_0000
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xfc00_0000;
}
