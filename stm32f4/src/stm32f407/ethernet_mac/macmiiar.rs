///Register `MACMIIAR` reader
pub type R = crate::R<MACMIIARrs>;
///Register `MACMIIAR` writer
pub type W = crate::W<MACMIIARrs>;
/**MII busy

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB {
    ///1: This bit is set to 1 by the application to indicate that a read or write access is in progress
    Busy = 1,
}
impl From<MB> for bool {
    #[inline(always)]
    fn from(variant: MB) -> Self {
        variant as u8 != 0
    }
}
///Field `MB` reader - MII busy
pub type MB_R = crate::BitReader<MB>;
impl MB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MB> {
        match self.bits {
            true => Some(MB::Busy),
            _ => None,
        }
    }
    ///This bit is set to 1 by the application to indicate that a read or write access is in progress
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == MB::Busy
    }
}
///Field `MB` writer - MII busy
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG, MB>;
impl<'a, REG> MB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit is set to 1 by the application to indicate that a read or write access is in progress
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(MB::Busy)
    }
}
/**MII write

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MW {
    ///0: Read operation
    Read = 0,
    ///1: Write operation
    Write = 1,
}
impl From<MW> for bool {
    #[inline(always)]
    fn from(variant: MW) -> Self {
        variant as u8 != 0
    }
}
///Field `MW` reader - MII write
pub type MW_R = crate::BitReader<MW>;
impl MW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MW {
        match self.bits {
            false => MW::Read,
            true => MW::Write,
        }
    }
    ///Read operation
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == MW::Read
    }
    ///Write operation
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == MW::Write
    }
}
///Field `MW` writer - MII write
pub type MW_W<'a, REG> = crate::BitWriter<'a, REG, MW>;
impl<'a, REG> MW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read operation
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(MW::Read)
    }
    ///Write operation
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(MW::Write)
    }
}
/**Clock range

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CR {
    ///0: 60-100MHz HCLK/42
    Cr60_100 = 0,
    ///1: 100-150 MHz HCLK/62
    Cr100_150 = 1,
    ///2: 20-35MHz HCLK/16
    Cr20_35 = 2,
    ///3: 35-60MHz HCLK/16
    Cr35_60 = 3,
    ///4: 150-168MHz HCLK/102
    Cr150_168 = 4,
}
impl From<CR> for u8 {
    #[inline(always)]
    fn from(variant: CR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CR {
    type Ux = u8;
}
impl crate::IsEnum for CR {}
///Field `CR` reader - Clock range
pub type CR_R = crate::FieldReader<CR>;
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CR> {
        match self.bits {
            0 => Some(CR::Cr60_100),
            1 => Some(CR::Cr100_150),
            2 => Some(CR::Cr20_35),
            3 => Some(CR::Cr35_60),
            4 => Some(CR::Cr150_168),
            _ => None,
        }
    }
    ///60-100MHz HCLK/42
    #[inline(always)]
    pub fn is_cr_60_100(&self) -> bool {
        *self == CR::Cr60_100
    }
    ///100-150 MHz HCLK/62
    #[inline(always)]
    pub fn is_cr_100_150(&self) -> bool {
        *self == CR::Cr100_150
    }
    ///20-35MHz HCLK/16
    #[inline(always)]
    pub fn is_cr_20_35(&self) -> bool {
        *self == CR::Cr20_35
    }
    ///35-60MHz HCLK/16
    #[inline(always)]
    pub fn is_cr_35_60(&self) -> bool {
        *self == CR::Cr35_60
    }
    ///150-168MHz HCLK/102
    #[inline(always)]
    pub fn is_cr_150_168(&self) -> bool {
        *self == CR::Cr150_168
    }
}
///Field `CR` writer - Clock range
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CR>;
impl<'a, REG> CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///60-100MHz HCLK/42
    #[inline(always)]
    pub fn cr_60_100(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Cr60_100)
    }
    ///100-150 MHz HCLK/62
    #[inline(always)]
    pub fn cr_100_150(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Cr100_150)
    }
    ///20-35MHz HCLK/16
    #[inline(always)]
    pub fn cr_20_35(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Cr20_35)
    }
    ///35-60MHz HCLK/16
    #[inline(always)]
    pub fn cr_35_60(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Cr35_60)
    }
    ///150-168MHz HCLK/102
    #[inline(always)]
    pub fn cr_150_168(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Cr150_168)
    }
}
///Field `MR` reader - MII register - select the desired MII register in the PHY device
pub type MR_R = crate::FieldReader;
///Field `MR` writer - MII register - select the desired MII register in the PHY device
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `PA` reader - PHY address - select which of possible 32 PHYs is being accessed
pub type PA_R = crate::FieldReader;
///Field `PA` writer - PHY address - select which of possible 32 PHYs is being accessed
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bit 0 - MII busy
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MII write
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - Clock range
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 6:10 - MII register - select the desired MII register in the PHY device
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMIIAR")
            .field("mb", &self.mb())
            .field("mw", &self.mw())
            .field("cr", &self.cr())
            .field("mr", &self.mr())
            .field("pa", &self.pa())
            .finish()
    }
}
impl W {
    ///Bit 0 - MII busy
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, MACMIIARrs> {
        MB_W::new(self, 0)
    }
    ///Bit 1 - MII write
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W<'_, MACMIIARrs> {
        MW_W::new(self, 1)
    }
    ///Bits 2:4 - Clock range
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, MACMIIARrs> {
        CR_W::new(self, 2)
    }
    ///Bits 6:10 - MII register - select the desired MII register in the PHY device
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W<'_, MACMIIARrs> {
        MR_W::new(self, 6)
    }
    ///Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, MACMIIARrs> {
        PA_W::new(self, 11)
    }
}
/**Ethernet MAC MII address register

You can [`read`](crate::Reg::read) this register and get [`macmiiar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiiar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#Ethernet_MAC:MACMIIAR)*/
pub struct MACMIIARrs;
impl crate::RegisterSpec for MACMIIARrs {
    type Ux = u32;
}
///`read()` method returns [`macmiiar::R`](R) reader structure
impl crate::Readable for MACMIIARrs {}
///`write(|w| ..)` method takes [`macmiiar::W`](W) writer structure
impl crate::Writable for MACMIIARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACMIIAR to value 0
impl crate::Resettable for MACMIIARrs {}
