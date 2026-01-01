///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
/**Option register bit 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OR_0 {
    ///0: LPTIM1 input 1 is connected to I/O
    Io = 0,
    ///1: LPTIM1 input 1 is connected to COMP1_OUT
    Comp1Out = 1,
}
impl From<OR_0> for bool {
    #[inline(always)]
    fn from(variant: OR_0) -> Self {
        variant as u8 != 0
    }
}
///Field `OR_0` reader - Option register bit 0
pub type OR_0_R = crate::BitReader<OR_0>;
impl OR_0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OR_0 {
        match self.bits {
            false => OR_0::Io,
            true => OR_0::Comp1Out,
        }
    }
    ///LPTIM1 input 1 is connected to I/O
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OR_0::Io
    }
    ///LPTIM1 input 1 is connected to COMP1_OUT
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == OR_0::Comp1Out
    }
}
///Field `OR_0` writer - Option register bit 0
pub type OR_0_W<'a, REG> = crate::BitWriter<'a, REG, OR_0>;
impl<'a, REG> OR_0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM1 input 1 is connected to I/O
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(OR_0::Io)
    }
    ///LPTIM1 input 1 is connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(OR_0::Comp1Out)
    }
}
/**Option register bit 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OR_1 {
    ///0: LPTIM1 input 2 is connected to I/O
    Io = 0,
    ///1: LPTIM1 input 2 is connected to COMP2_OUT
    Comp2Out = 1,
}
impl From<OR_1> for bool {
    #[inline(always)]
    fn from(variant: OR_1) -> Self {
        variant as u8 != 0
    }
}
///Field `OR_1` reader - Option register bit 1
pub type OR_1_R = crate::BitReader<OR_1>;
impl OR_1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OR_1 {
        match self.bits {
            false => OR_1::Io,
            true => OR_1::Comp2Out,
        }
    }
    ///LPTIM1 input 2 is connected to I/O
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OR_1::Io
    }
    ///LPTIM1 input 2 is connected to COMP2_OUT
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == OR_1::Comp2Out
    }
}
///Field `OR_1` writer - Option register bit 1
pub type OR_1_W<'a, REG> = crate::BitWriter<'a, REG, OR_1>;
impl<'a, REG> OR_1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM1 input 2 is connected to I/O
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(OR_1::Io)
    }
    ///LPTIM1 input 2 is connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(OR_1::Comp2Out)
    }
}
impl R {
    ///Bit 0 - Option register bit 0
    #[inline(always)]
    pub fn or_0(&self) -> OR_0_R {
        OR_0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option register bit 1
    #[inline(always)]
    pub fn or_1(&self) -> OR_1_R {
        OR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("or_1", &self.or_1())
            .field("or_0", &self.or_0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option register bit 0
    #[inline(always)]
    pub fn or_0(&mut self) -> OR_0_W<'_, ORrs> {
        OR_0_W::new(self, 0)
    }
    ///Bit 1 - Option register bit 1
    #[inline(always)]
    pub fn or_1(&mut self) -> OR_1_W<'_, ORrs> {
        OR_1_W::new(self, 1)
    }
}
/**option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#LPTIM1:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
