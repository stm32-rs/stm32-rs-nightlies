///Register `AFRL` reader
pub type R = crate::R<AFRLrs>;
///Register `AFRL` writer
pub type W = crate::W<AFRLrs>;
/**Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALTERNATE_FUNCTION {
    ///0: AF0
    Af0 = 0,
    ///1: AF1
    Af1 = 1,
    ///2: AF2
    Af2 = 2,
    ///3: AF3
    Af3 = 3,
    ///4: AF4
    Af4 = 4,
    ///5: AF5
    Af5 = 5,
    ///6: AF6
    Af6 = 6,
    ///7: AF7
    Af7 = 7,
    ///8: AF8
    Af8 = 8,
    ///9: AF9
    Af9 = 9,
    ///10: AF10
    Af10 = 10,
    ///11: AF11
    Af11 = 11,
    ///12: AF12
    Af12 = 12,
    ///13: AF13
    Af13 = 13,
    ///14: AF14
    Af14 = 14,
    ///15: AF15
    Af15 = 15,
}
impl From<ALTERNATE_FUNCTION> for u8 {
    #[inline(always)]
    fn from(variant: ALTERNATE_FUNCTION) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALTERNATE_FUNCTION {
    type Ux = u8;
}
impl crate::IsEnum for ALTERNATE_FUNCTION {}
///Field `AFSEL3` reader - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
pub type AFSEL3_R = crate::FieldReader<ALTERNATE_FUNCTION>;
impl AFSEL3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALTERNATE_FUNCTION {
        match self.bits {
            0 => ALTERNATE_FUNCTION::Af0,
            1 => ALTERNATE_FUNCTION::Af1,
            2 => ALTERNATE_FUNCTION::Af2,
            3 => ALTERNATE_FUNCTION::Af3,
            4 => ALTERNATE_FUNCTION::Af4,
            5 => ALTERNATE_FUNCTION::Af5,
            6 => ALTERNATE_FUNCTION::Af6,
            7 => ALTERNATE_FUNCTION::Af7,
            8 => ALTERNATE_FUNCTION::Af8,
            9 => ALTERNATE_FUNCTION::Af9,
            10 => ALTERNATE_FUNCTION::Af10,
            11 => ALTERNATE_FUNCTION::Af11,
            12 => ALTERNATE_FUNCTION::Af12,
            13 => ALTERNATE_FUNCTION::Af13,
            14 => ALTERNATE_FUNCTION::Af14,
            15 => ALTERNATE_FUNCTION::Af15,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af0
    }
    ///AF1
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af1
    }
    ///AF2
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af2
    }
    ///AF3
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af3
    }
    ///AF4
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af4
    }
    ///AF5
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af5
    }
    ///AF6
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af6
    }
    ///AF7
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af7
    }
    ///AF8
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af8
    }
    ///AF9
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af9
    }
    ///AF10
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af10
    }
    ///AF11
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af11
    }
    ///AF12
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af12
    }
    ///AF13
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af13
    }
    ///AF14
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af14
    }
    ///AF15
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == ALTERNATE_FUNCTION::Af15
    }
}
///Field `AFSEL3` writer - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
pub type AFSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ALTERNATE_FUNCTION, crate::Safe>;
impl<'a, REG> AFSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut crate::W<REG> {
        self.variant(ALTERNATE_FUNCTION::Af15)
    }
}
impl R {
    ///Bits 12:15 - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRL")
            .field("afsel3", &self.afsel3())
            .finish()
    }
}
impl W {
    ///Bits 12:15 - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn afsel3(&mut self) -> AFSEL3_W<'_, AFRLrs> {
        AFSEL3_W::new(self, 12)
    }
}
/**GPIO port H alternate function low register

You can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOH:AFRL)*/
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
///`read()` method returns [`afrl::R`](R) reader structure
impl crate::Readable for AFRLrs {}
///`write(|w| ..)` method takes [`afrl::W`](W) writer structure
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRL to value 0
impl crate::Resettable for AFRLrs {}
