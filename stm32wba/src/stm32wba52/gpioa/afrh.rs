///Register `AFRH` reader
pub type R = crate::R<AFRHrs>;
///Register `AFRH` writer
pub type W = crate::W<AFRHrs>;
/**Alternate function selection for port I/O pin 8

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
///Field `AFSEL8` reader - Alternate function selection for port I/O pin 8
pub type AFSEL8_R = crate::FieldReader<ALTERNATE_FUNCTION>;
impl AFSEL8_R {
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
///Field `AFSEL8` writer - Alternate function selection for port I/O pin 8
pub type AFSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ALTERNATE_FUNCTION, crate::Safe>;
impl<'a, REG> AFSEL8_W<'a, REG>
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
///Field `AFSEL9` reader - Alternate function selection for port I/O pin 9
pub use AFSEL8_R as AFSEL9_R;
///Field `AFSEL10` reader - Alternate function selection for port I/O pin 10
pub use AFSEL8_R as AFSEL10_R;
///Field `AFSEL11` reader - Alternate function selection for port I/O pin 11
pub use AFSEL8_R as AFSEL11_R;
///Field `AFSEL12` reader - Alternate function selection for port I/O pin 12
pub use AFSEL8_R as AFSEL12_R;
///Field `AFSEL13` reader - Alternate function selection for port I/O pin 13
pub use AFSEL8_R as AFSEL13_R;
///Field `AFSEL14` reader - Alternate function selection for port I/O pin 14
pub use AFSEL8_R as AFSEL14_R;
///Field `AFSEL15` reader - Alternate function selection for port I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SEC15.
pub use AFSEL8_R as AFSEL15_R;
///Field `AFSEL9` writer - Alternate function selection for port I/O pin 9
pub use AFSEL8_W as AFSEL9_W;
///Field `AFSEL10` writer - Alternate function selection for port I/O pin 10
pub use AFSEL8_W as AFSEL10_W;
///Field `AFSEL11` writer - Alternate function selection for port I/O pin 11
pub use AFSEL8_W as AFSEL11_W;
///Field `AFSEL12` writer - Alternate function selection for port I/O pin 12
pub use AFSEL8_W as AFSEL12_W;
///Field `AFSEL13` writer - Alternate function selection for port I/O pin 13
pub use AFSEL8_W as AFSEL13_W;
///Field `AFSEL14` writer - Alternate function selection for port I/O pin 14
pub use AFSEL8_W as AFSEL14_W;
///Field `AFSEL15` writer - Alternate function selection for port I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SEC15.
pub use AFSEL8_W as AFSEL15_W;
impl R {
    ///Bits 0:3 - Alternate function selection for port I/O pin 8
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port I/O pin 9
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port I/O pin 10
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port I/O pin 11
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port I/O pin 12
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port I/O pin 13
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port I/O pin 14
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SEC15.
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afsel8", &self.afsel8())
            .field("afsel9", &self.afsel9())
            .field("afsel10", &self.afsel10())
            .field("afsel11", &self.afsel11())
            .field("afsel12", &self.afsel12())
            .field("afsel13", &self.afsel13())
            .field("afsel14", &self.afsel14())
            .field("afsel15", &self.afsel15())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port I/O pin 8
    #[inline(always)]
    pub fn afsel8(&mut self) -> AFSEL8_W<'_, AFRHrs> {
        AFSEL8_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port I/O pin 9
    #[inline(always)]
    pub fn afsel9(&mut self) -> AFSEL9_W<'_, AFRHrs> {
        AFSEL9_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port I/O pin 10
    #[inline(always)]
    pub fn afsel10(&mut self) -> AFSEL10_W<'_, AFRHrs> {
        AFSEL10_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port I/O pin 11
    #[inline(always)]
    pub fn afsel11(&mut self) -> AFSEL11_W<'_, AFRHrs> {
        AFSEL11_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port I/O pin 12
    #[inline(always)]
    pub fn afsel12(&mut self) -> AFSEL12_W<'_, AFRHrs> {
        AFSEL12_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port I/O pin 13
    #[inline(always)]
    pub fn afsel13(&mut self) -> AFSEL13_W<'_, AFRHrs> {
        AFSEL13_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port I/O pin 14
    #[inline(always)]
    pub fn afsel14(&mut self) -> AFSEL14_W<'_, AFRHrs> {
        AFSEL14_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOA SEC15.
    #[inline(always)]
    pub fn afsel15(&mut self) -> AFSEL15_W<'_, AFRHrs> {
        AFSEL15_W::new(self, 28)
    }
}
/**GPIO port A alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOA:AFRH)*/
pub struct AFRHrs;
impl crate::RegisterSpec for AFRHrs {
    type Ux = u32;
}
///`read()` method returns [`afrh::R`](R) reader structure
impl crate::Readable for AFRHrs {}
///`write(|w| ..)` method takes [`afrh::W`](W) writer structure
impl crate::Writable for AFRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRHrs {}
