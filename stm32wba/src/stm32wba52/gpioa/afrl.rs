///Register `AFRL` reader
pub type R = crate::R<AFRLrs>;
///Register `AFRL` writer
pub type W = crate::W<AFRLrs>;
/**Alternate function selection for port I/O pin 0

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
///Field `AFSEL0` reader - Alternate function selection for port I/O pin 0
pub type AFSEL0_R = crate::FieldReader<ALTERNATE_FUNCTION>;
impl AFSEL0_R {
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
///Field `AFSEL0` writer - Alternate function selection for port I/O pin 0
pub type AFSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ALTERNATE_FUNCTION, crate::Safe>;
impl<'a, REG> AFSEL0_W<'a, REG>
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
///Field `AFSEL1` reader - Alternate function selection for port I/O pin 1
pub use AFSEL0_R as AFSEL1_R;
///Field `AFSEL2` reader - Alternate function selection for port I/O pin 2
pub use AFSEL0_R as AFSEL2_R;
///Field `AFSEL3` reader - Alternate function selection for port I/O pin 3
pub use AFSEL0_R as AFSEL3_R;
///Field `AFSEL5` reader - Alternate function selection for port I/O pin 5
pub use AFSEL0_R as AFSEL5_R;
///Field `AFSEL6` reader - Alternate function selection for port I/O pin 6
pub use AFSEL0_R as AFSEL6_R;
///Field `AFSEL7` reader - Alternate function selection for port I/O pin 7 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SECy.
pub use AFSEL0_R as AFSEL7_R;
///Field `AFSEL1` writer - Alternate function selection for port I/O pin 1
pub use AFSEL0_W as AFSEL1_W;
///Field `AFSEL2` writer - Alternate function selection for port I/O pin 2
pub use AFSEL0_W as AFSEL2_W;
///Field `AFSEL3` writer - Alternate function selection for port I/O pin 3
pub use AFSEL0_W as AFSEL3_W;
///Field `AFSEL5` writer - Alternate function selection for port I/O pin 5
pub use AFSEL0_W as AFSEL5_W;
///Field `AFSEL6` writer - Alternate function selection for port I/O pin 6
pub use AFSEL0_W as AFSEL6_W;
///Field `AFSEL7` writer - Alternate function selection for port I/O pin 7 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SECy.
pub use AFSEL0_W as AFSEL7_W;
impl R {
    ///Bits 0:3 - Alternate function selection for port I/O pin 0
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port I/O pin 1
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port I/O pin 2
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port I/O pin 3
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port I/O pin 5
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port I/O pin 6
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port I/O pin 7 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SECy.
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRL")
            .field("afsel0", &self.afsel0())
            .field("afsel1", &self.afsel1())
            .field("afsel2", &self.afsel2())
            .field("afsel3", &self.afsel3())
            .field("afsel5", &self.afsel5())
            .field("afsel6", &self.afsel6())
            .field("afsel7", &self.afsel7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port I/O pin 0
    #[inline(always)]
    pub fn afsel0(&mut self) -> AFSEL0_W<'_, AFRLrs> {
        AFSEL0_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port I/O pin 1
    #[inline(always)]
    pub fn afsel1(&mut self) -> AFSEL1_W<'_, AFRLrs> {
        AFSEL1_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port I/O pin 2
    #[inline(always)]
    pub fn afsel2(&mut self) -> AFSEL2_W<'_, AFRLrs> {
        AFSEL2_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port I/O pin 3
    #[inline(always)]
    pub fn afsel3(&mut self) -> AFSEL3_W<'_, AFRLrs> {
        AFSEL3_W::new(self, 12)
    }
    ///Bits 20:23 - Alternate function selection for port I/O pin 5
    #[inline(always)]
    pub fn afsel5(&mut self) -> AFSEL5_W<'_, AFRLrs> {
        AFSEL5_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port I/O pin 6
    #[inline(always)]
    pub fn afsel6(&mut self) -> AFSEL6_W<'_, AFRLrs> {
        AFSEL6_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port I/O pin 7 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SECy.
    #[inline(always)]
    pub fn afsel7(&mut self) -> AFSEL7_W<'_, AFRLrs> {
        AFSEL7_W::new(self, 28)
    }
}
/**GPIO port A alternate function low register

You can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOA:AFRL)*/
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
