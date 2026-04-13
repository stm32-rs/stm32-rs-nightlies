///Register `I2C_OAR2` reader
pub type R = crate::R<I2C_OAR2rs>;
///Register `I2C_OAR2` writer
pub type W = crate::W<I2C_OAR2rs>;
///Field `OA2` reader - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN = 0.
pub type OA2_R = crate::FieldReader;
///Field `OA2` writer - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN = 0.
pub type OA2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Own address 2 masks

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OA2MSK {
    ///0: No mask
    B0x0 = 0,
    ///1: OA2\[1\] is masked and don t care. Only OA2\[7:2\] are compared.
    B0x1 = 1,
    ///2: OA2\[2:1\] are masked and don t care. Only OA2\[7:3\] are compared.
    B0x2 = 2,
    ///3: OA2\[3:1\] are masked and don t care. Only OA2\[7:4\] are compared.
    B0x3 = 3,
    ///4: OA2\[4:1\] are masked and don t care. Only OA2\[7:5\] are compared.
    B0x4 = 4,
    ///5: OA2\[5:1\] are masked and don t care. Only OA2\[7:6\] are compared.
    B0x5 = 5,
    ///6: OA2\[6:1\] are masked and don t care. Only OA2\[7\] is compared.
    B0x6 = 6,
    ///7: OA2\[7:1\] are masked and don t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged.
    B0x7 = 7,
}
impl From<OA2MSK> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OA2MSK {
    type Ux = u8;
}
impl crate::IsEnum for OA2MSK {}
///Field `OA2MSK` reader - Own address 2 masks
pub type OA2MSK_R = crate::FieldReader<OA2MSK>;
impl OA2MSK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OA2MSK {
        match self.bits {
            0 => OA2MSK::B0x0,
            1 => OA2MSK::B0x1,
            2 => OA2MSK::B0x2,
            3 => OA2MSK::B0x3,
            4 => OA2MSK::B0x4,
            5 => OA2MSK::B0x5,
            6 => OA2MSK::B0x6,
            7 => OA2MSK::B0x7,
            _ => unreachable!(),
        }
    }
    ///No mask
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA2MSK::B0x0
    }
    ///OA2\[1\] is masked and don t care. Only OA2\[7:2\] are compared.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA2MSK::B0x1
    }
    ///OA2\[2:1\] are masked and don t care. Only OA2\[7:3\] are compared.
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OA2MSK::B0x2
    }
    ///OA2\[3:1\] are masked and don t care. Only OA2\[7:4\] are compared.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OA2MSK::B0x3
    }
    ///OA2\[4:1\] are masked and don t care. Only OA2\[7:5\] are compared.
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OA2MSK::B0x4
    }
    ///OA2\[5:1\] are masked and don t care. Only OA2\[7:6\] are compared.
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OA2MSK::B0x5
    }
    ///OA2\[6:1\] are masked and don t care. Only OA2\[7\] is compared.
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OA2MSK::B0x6
    }
    ///OA2\[7:1\] are masked and don t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged.
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OA2MSK::B0x7
    }
}
///Field `OA2MSK` writer - Own address 2 masks
pub type OA2MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OA2MSK, crate::Safe>;
impl<'a, REG> OA2MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No mask
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x0)
    }
    ///OA2\[1\] is masked and don t care. Only OA2\[7:2\] are compared.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x1)
    }
    ///OA2\[2:1\] are masked and don t care. Only OA2\[7:3\] are compared.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x2)
    }
    ///OA2\[3:1\] are masked and don t care. Only OA2\[7:4\] are compared.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x3)
    }
    ///OA2\[4:1\] are masked and don t care. Only OA2\[7:5\] are compared.
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x4)
    }
    ///OA2\[5:1\] are masked and don t care. Only OA2\[7:6\] are compared.
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x5)
    }
    ///OA2\[6:1\] are masked and don t care. Only OA2\[7\] is compared.
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x6)
    }
    ///OA2\[7:1\] are masked and don t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged.
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::B0x7)
    }
}
/**Own address 2 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA2EN {
    ///0: Own address 2 disabled. The received slave address OA2 is NACKed.
    B0x0 = 0,
    ///1: Own address 2 enabled. The received slave address OA2 is ACKed.
    B0x1 = 1,
}
impl From<OA2EN> for bool {
    #[inline(always)]
    fn from(variant: OA2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OA2EN` reader - Own address 2 enable
pub type OA2EN_R = crate::BitReader<OA2EN>;
impl OA2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OA2EN {
        match self.bits {
            false => OA2EN::B0x0,
            true => OA2EN::B0x1,
        }
    }
    ///Own address 2 disabled. The received slave address OA2 is NACKed.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA2EN::B0x0
    }
    ///Own address 2 enabled. The received slave address OA2 is ACKed.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA2EN::B0x1
    }
}
///Field `OA2EN` writer - Own address 2 enable
pub type OA2EN_W<'a, REG> = crate::BitWriter<'a, REG, OA2EN>;
impl<'a, REG> OA2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Own address 2 disabled. The received slave address OA2 is NACKed.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN::B0x0)
    }
    ///Own address 2 enabled. The received slave address OA2 is ACKed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN::B0x1)
    }
}
impl R {
    ///Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN = 0.
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bits 8:10 - Own address 2 masks
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 15 - Own address 2 enable
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_OAR2")
            .field("oa2", &self.oa2())
            .field("oa2msk", &self.oa2msk())
            .field("oa2en", &self.oa2en())
            .finish()
    }
}
impl W {
    ///Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN = 0.
    #[inline(always)]
    pub fn oa2(&mut self) -> OA2_W<'_, I2C_OAR2rs> {
        OA2_W::new(self, 1)
    }
    ///Bits 8:10 - Own address 2 masks
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W<'_, I2C_OAR2rs> {
        OA2MSK_W::new(self, 8)
    }
    ///Bit 15 - Own address 2 enable
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W<'_, I2C_OAR2rs> {
        OA2EN_W::new(self, 15)
    }
}
/**I2C own address 2 register

You can [`read`](crate::Reg::read) this register and get [`i2c_oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#I2C1:I2C_OAR2)*/
pub struct I2C_OAR2rs;
impl crate::RegisterSpec for I2C_OAR2rs {
    type Ux = u32;
}
///`read()` method returns [`i2c_oar2::R`](R) reader structure
impl crate::Readable for I2C_OAR2rs {}
///`write(|w| ..)` method takes [`i2c_oar2::W`](W) writer structure
impl crate::Writable for I2C_OAR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_OAR2 to value 0
impl crate::Resettable for I2C_OAR2rs {}
