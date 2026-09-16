///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
/**Port H configuration I/O pin 3 These bits are written by software to configure the I/O mode. Access can be protected by GPIOH SEC3.

Value on reset: 0*/
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
///Field `MODE3` reader - Port H configuration I/O pin 3 These bits are written by software to configure the I/O mode. Access can be protected by GPIOH SEC3.
pub type MODE3_R = crate::FieldReader<MODE>;
impl MODE3_R {
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
///Field `MODE3` writer - Port H configuration I/O pin 3 These bits are written by software to configure the I/O mode. Access can be protected by GPIOH SEC3.
pub type MODE3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODE3_W<'a, REG>
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
impl R {
    ///Bits 6:7 - Port H configuration I/O pin 3 These bits are written by software to configure the I/O mode. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("mode3", &self.mode3())
            .finish()
    }
}
impl W {
    ///Bits 6:7 - Port H configuration I/O pin 3 These bits are written by software to configure the I/O mode. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W<'_, MODERrs> {
        MODE3_W::new(self, 6)
    }
}
/**GPIO port H mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOH:MODER)*/
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
///`reset()` method sets MODER to value 0xc000
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xc000;
}
