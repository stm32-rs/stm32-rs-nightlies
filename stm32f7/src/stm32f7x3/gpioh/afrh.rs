///Register `AFRH` reader
pub type R = crate::R<AFRHrs>;
///Register `AFRH` writer
pub type W = crate::W<AFRHrs>;
/**Alternate function selection for port x bit y (y = 8..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFRH8 {
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
impl From<AFRH8> for u8 {
    #[inline(always)]
    fn from(variant: AFRH8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFRH8 {
    type Ux = u8;
}
impl crate::IsEnum for AFRH8 {}
///Field `AFR(H8,H9,H10,H11,H12,H13,H14,H15)` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFR_R = crate::FieldReader<AFRH8>;
impl AFR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFRH8 {
        match self.bits {
            0 => AFRH8::Af0,
            1 => AFRH8::Af1,
            2 => AFRH8::Af2,
            3 => AFRH8::Af3,
            4 => AFRH8::Af4,
            5 => AFRH8::Af5,
            6 => AFRH8::Af6,
            7 => AFRH8::Af7,
            8 => AFRH8::Af8,
            9 => AFRH8::Af9,
            10 => AFRH8::Af10,
            11 => AFRH8::Af11,
            12 => AFRH8::Af12,
            13 => AFRH8::Af13,
            14 => AFRH8::Af14,
            15 => AFRH8::Af15,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRH8::Af0
    }
    ///AF1
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRH8::Af1
    }
    ///AF2
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRH8::Af2
    }
    ///AF3
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRH8::Af3
    }
    ///AF4
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRH8::Af4
    }
    ///AF5
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRH8::Af5
    }
    ///AF6
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRH8::Af6
    }
    ///AF7
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRH8::Af7
    }
    ///AF8
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFRH8::Af8
    }
    ///AF9
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFRH8::Af9
    }
    ///AF10
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFRH8::Af10
    }
    ///AF11
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFRH8::Af11
    }
    ///AF12
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFRH8::Af12
    }
    ///AF13
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFRH8::Af13
    }
    ///AF14
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFRH8::Af14
    }
    ///AF15
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFRH8::Af15
    }
}
///Field `AFR(H8,H9,H10,H11,H12,H13,H14,H15)` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFRH8, crate::Safe>;
impl<'a, REG> AFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut crate::W<REG> {
        self.variant(AFRH8::Af15)
    }
}
impl R {
    ///Alternate function selection for port x bit y (y = 8..15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFRH8` field.</div>
    #[inline(always)]
    pub fn afr(&self, n: u8) -> AFR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afr_iter(&self) -> impl Iterator<Item = AFR_R> + '_ {
        (0..8).map(move |n| AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh8(&self) -> AFR_R {
        AFR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh9(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh10(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh11(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh12(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh13(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh14(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh15(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afrh8", &self.afrh8())
            .field("afrh9", &self.afrh9())
            .field("afrh10", &self.afrh10())
            .field("afrh11", &self.afrh11())
            .field("afrh12", &self.afrh12())
            .field("afrh13", &self.afrh13())
            .field("afrh14", &self.afrh14())
            .field("afrh15", &self.afrh15())
            .finish()
    }
}
impl W {
    ///Alternate function selection for port x bit y (y = 8..15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFRH8` field.</div>
    #[inline(always)]
    pub fn afr(&mut self, n: u8) -> AFR_W<AFRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_W::new(self, n * 4)
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh8(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh9(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh10(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh11(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh12(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh13(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh14(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh15(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 28)
    }
}
/**GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F7x3.html#GPIOH:AFRH)*/
pub struct AFRHrs;
impl crate::RegisterSpec for AFRHrs {
    type Ux = u32;
}
///`read()` method returns [`afrh::R`](R) reader structure
impl crate::Readable for AFRHrs {}
///`write(|w| ..)` method takes [`afrh::W`](W) writer structure
impl crate::Writable for AFRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRHrs {
    const RESET_VALUE: u32 = 0;
}
